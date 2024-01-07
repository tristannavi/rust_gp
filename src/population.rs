use std::thread;

use rand::Rng;

use crate::chromosome::Chromosome;
use crate::io::Dataset;
use crate::island::IslandParameters;

pub struct PopulationParameters {
    pub generations: usize,
    pub population_size: usize,
    pub num_genes: usize,
    pub mut_chance: f64,
    pub crossover_chance: f64,
    pub island_parameters: IslandParameters,
}

pub trait PopulationTraits {
    fn mate(&self, num_variables: usize, crossover_chance: f64, mutation_chance: f64) -> (Population, f64);
    fn find_best_min(self) -> Chromosome;
    fn new() -> Population;
    fn tournament_selection(&self) -> &Chromosome;
    fn get_random_chromosome(&self) -> &Chromosome;
    fn all_accessed(&mut self);
    fn initialize(size: usize, num_genes: usize, dataset: &Dataset) -> Population;
    fn evaluate(&mut self, dataset: &Dataset);
}

pub type Population = Vec<Chromosome>;
pub type Island = Population;

impl PopulationTraits for Population {
    fn mate(&self, num_variables: usize, crossover_chance: f64, mutation_chance: f64) -> (Population, f64) {
        let mut new_population = Population::new();

        // Elitism by adding the best out of the entire population to the new population
        let best = self.clone().find_best_min();
        let best_fitness = best.fitness_value;
        new_population.push(best);
        for _i in (1..self.len()).step_by(2) {
            let mut offspring_one = self.tournament_selection().clone();
            let mut offspring_two = self.tournament_selection().clone();

            if rand::thread_rng().gen_bool(crossover_chance) { offspring_one.cross_with(&mut offspring_two, None); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_one.mutate(num_variables); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_two.mutate(num_variables); }

            new_population.push(offspring_one);
            new_population.push(offspring_two);
        }
        return (new_population, best_fitness);
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
    fn find_best_min(self) -> Chromosome {
        let mut best = Chromosome::new();
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

    fn evaluate(&mut self, dataset: &Dataset) {
        thread::scope(|s| {
            for mut i in self {
                s.spawn(|| {
                    i.evaluate_fitness_mse(&dataset);
                });
            }
        });
    }
}