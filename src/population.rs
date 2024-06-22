use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::chromosome::Chromosome;
use crate::io::Dataset;

pub struct PopulationParameters {
    pub generations: usize,
    pub population_size: usize,
    pub num_genes: usize,
    pub mut_chance: f64,
    pub crossover_chance: f64,
}

pub trait PopulationTraits {
    fn mate(&mut self, num_variables: usize, crossover_chance: f64, mutation_chance: f64, dataset: &Dataset) -> f64;
    fn find_best_min(&mut self);
    fn tournament_selection(&self) -> &Chromosome;
    fn get_random_chromosome(&self) -> &Chromosome;
    fn all_accessed(&mut self);
    fn initialize(size: usize, num_genes: usize, dataset: &Dataset) -> Population;
    fn evaluate(&mut self, dataset: &Dataset);
    fn len(&self) -> usize;
}

pub struct Population {
    pub(crate) population: Vec<Chromosome>,
    pub(crate) best: Chromosome,
}

impl PopulationTraits for Population {
    /// Mate the individuals in the population to create a new population.
    ///
    /// # Arguments
    ///
    /// * `num_variables` - The number of variables in the dataset.
    /// * `crossover_chance` - The probability of crossover.
    /// * `mutation_chance` - The probability of mutation.
    ///
    /// # Returns
    ///
    /// A tuple containing the new population and the fitness value of the best individual.
    /// Also replaces the population in memory
    fn mate(&mut self, num_variables: usize, crossover_chance: f64, mutation_chance: f64, dataset: &Dataset) -> f64 {
        /// Takes a population, crossover chance, mutation chance, and number of variables as input
        /// and returns a tuple of two new offspring chromosomes.
        ///
        /// # Arguments
        ///
        /// * `population` - A reference to a `Population` instance.
        /// * `crossover_chance` - The chance of crossover as a floating-point number between 0 and 1.
        /// * `mutation_chance` - The chance of mutation as a floating-point number between 0 and 1.
        /// * `num_variables` - The number of variables in the chromosomes.
        ///
        /// # Returns
        ///
        /// A tuple containing two `Chromosome` instances representing the new offspring.
        ///
        /// # Examples
        ///
        /// ```
        /// let population = Population::new();
        /// let crossover_chance = 0.8;
        /// let mutation_chance = 0.1;
        /// let num_variables = 5;
        ///
        /// let (offspring_one, offspring_two) = get_new_offspring(&population, crossover_chance, mutation_chance, num_variables);
        ///
        /// assert_eq!(offspring_one.num_variables(), num_variables);
        /// assert_eq!(offspring_two.num_variables(), num_variables);
        /// ```
        fn get_new_offspring(population: &Population, crossover_chance: f64, mutation_chance: f64, num_variables: usize) -> (Chromosome, Chromosome) {
            let mut offspring_one = population.tournament_selection().clone();
            let mut offspring_two = population.tournament_selection().clone();

            if rand::thread_rng().gen_bool(crossover_chance) { offspring_one.cross_with(&mut offspring_two, None); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_one.mutate(num_variables); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_two.mutate(num_variables); }

            return (offspring_one, offspring_two);
        }

        let mut new_population: Vec<Chromosome> = (1..self.population.len())
            .into_par_iter()
            .step_by(2)
            .flat_map(|_| {
                let (offspring_one, offspring_two) = get_new_offspring(self, crossover_chance, mutation_chance, num_variables);
                return vec![offspring_one, offspring_two];
            })
            .collect();

        // Elitism by adding the best out of the entire population to the new population
        new_population.push(self.best.clone()); // Population best has not been updated yet

        // Replace current Population with new Population
        self.population = new_population;

        self.evaluate(dataset);
        return self.best.fitness_value;
    }

    /// Returns the chromosome with the minimum fitness value in the given `Population`.
    ///
    /// # Example
    ///
    /// ```
    /// let population: Vec<Chromosome> = vec![...];
    /// let best_chromosome = population.find_best_min();
    /// ```
    ///
    /// # Returns
    ///
    /// The chromosome with the minimum fitness value.
    fn find_best_min(&mut self) {
        for i in &self.population {
            if i.fitness_value < self.best.fitness_value {
                self.best = i.clone();
            }
        }
    }

    /// Performs tournament selection with k = 2 for the population
    ///
    /// Randomly selects two chromosomes and returns the one with the minimum fitness value.
    /// If both chromosomes have the same fitness value, the first chromosome is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm::Population;
    ///
    /// let population = Population::new();
    /// let best_chromosome = population.best_min_random();
    /// println!("Best chromosome: {:?}", best_chromosome);
    /// ```
    fn tournament_selection(&self) -> &Chromosome {
        let c1 = self.get_random_chromosome();
        let c2 = self.get_random_chromosome();
        return if c1.fitness_value < c2.fitness_value { c1 } else { c2 };
    }

    /// Returns a reference to a randomly selected `Chromosome` from the `self` vector.
    fn get_random_chromosome(&self) -> &Chromosome {
        return &self.population[rand::thread_rng().gen_range(0..self.len())];
    }

    fn all_accessed(&mut self) {
        let mut count = 0;
        for mut c in &mut self.population {
            if !c.accessed {
                count += 1;
            }
            c.accessed = false;
        }
        assert_eq!(count, 0, "Not all chromosomes in this population were evaluated");
    }

    fn initialize(size: usize, num_genes: usize, dataset: &Dataset) -> Population {
        let mut population = Population {
            population: (0..size).into_iter().map(|_| Chromosome::new_x(num_genes, dataset[0].len() - 2)).collect(),
            best: Chromosome::new(),
        };
        population.find_best_min();
        return population;
    }

    /// Evaluates the fitness of each chromosome in the population using the mean squared error (MSE)
    /// as the fitness function.
    ///
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to a `Dataset` containing the data to evaluate the chromosomes
    /// against.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm::Population;
    ///
    /// let mut population = Population::new();
    /// let dataset = vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![4.0, 5.0, 6.0],
    ///     vec![7.0, 8.0, 9.0]
    /// ];
    /// population.evaluate(&dataset);
    /// ```
    ///
    /// # Returns
    ///
    /// None.
    fn evaluate(&mut self, dataset: &Dataset) {
        // let min = self.population.par_iter_mut().map(|mut i| { let _ = i.evaluate_fitness_mse(dataset); }).min();
        self.population.par_iter_mut().for_each(|mut i| { let _ = i.evaluate_fitness_mse(dataset); });
        self.find_best_min();
    }

    /// Returns the length of the population.
    ///
    /// # Returns
    ///
    /// The number of elements in the population.
    ///
    /// # Examples
    ///
    /// ```
    /// let population = vec![1, 2, 3];
    /// let count = len(&population);
    /// assert_eq!(count, 3);
    /// ```
    fn len(&self) -> usize {
        return self.population.len();
    }
}