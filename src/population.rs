use std::thread;

use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

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
    fn find_best_min(&self) -> Chromosome;
    fn new() -> Population;
    fn tournament_selection(&self) -> &Chromosome;
    fn get_random_chromosome(&self) -> &Chromosome;
    fn all_accessed(&mut self);
    fn initialize(size: usize, num_genes: usize, dataset: &Dataset) -> Population;
    fn evaluate(&mut self, dataset: &Dataset);
}

pub type Population = Vec<Chromosome>;

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

        // Elitism by adding the best out of the entire population to the new population
        let best = self.find_best_min();
        let best_fitness = best.fitness_value;

        let mut new_population: Population = (1..self.len())
            .into_par_iter()
            .step_by(2)
            .flat_map(|_| {
                let (offspring_one, offspring_two) = get_new_offspring(self, crossover_chance, mutation_chance, num_variables);
                return vec![offspring_one, offspring_two];
            })
            .collect();

        // Add the best individual to the new population
        new_population.push(best);

        // Replace current Population with new Population
        let _ = std::mem::replace(self, new_population);
        self.evaluate(dataset);
        return self.find_best_min().fitness_value;
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
    fn find_best_min(&self) -> Chromosome {
        let mut best = &Chromosome::new();
        for i in self {
            if i.fitness_value < best.fitness_value {
                best = i;
            }
        }
        return best.clone();
    }

    /// Constructs a new `Population` object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std_genetics::Population;
    ///
    /// let population = Population::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Population` object, initially empty.
    fn new() -> Population {
        return vec![] as Population;
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
        return &self[rand::thread_rng().gen_range(0..self.len())];
    }

    fn all_accessed(&mut self) {
        let mut count = 0;
        for c in self {
            if !c.accessed {
                count += 1;
            }
            c.accessed = false;
        }
        assert_eq!(count, 0, "Not all chromosomes in this population were evaluated");
    }

    fn initialize(size: usize, num_genes: usize, dataset: &Dataset) -> Population {
        let mut population = vec![];
        for _p in 0..size {
            population.push(Chromosome::new_x(num_genes, dataset[0].len() - 2))
        }

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
        self.par_iter_mut().for_each(|mut i| { let _ = i.evaluate_fitness_mse(dataset); });
    }
}