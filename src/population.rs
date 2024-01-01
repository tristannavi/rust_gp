use rand::Rng;

use crate::chromosome::Chromosome;

pub trait Mate {
    fn mate(&self, num_variables: usize, crossover_chance: f64, mutation_chance: f64) -> Population;
    fn find_best_min(self) -> Chromosome;
    fn new() -> Population;
    fn tournament_selection(&self) -> &Chromosome;
    fn get_random_chromosome(&self) -> &Chromosome;
}

pub type Population = Vec<Chromosome>;

impl Mate for Population {
    fn mate(&self, num_variables: usize, crossover_chance: f64, mutation_chance: f64) -> Population {
        let mut new_population = Population::new();

        // Elitism by adding the best out of the entire population to the new population
        let best = self.clone().find_best_min();
        new_population.push(best);
        for i in (1..self.len()).step_by(2) {
            let mut offspring_one = self.tournament_selection().clone();
            let mut offspring_two = self.tournament_selection().clone();

            if rand::thread_rng().gen_bool(crossover_chance) { offspring_one.cross_with(&mut offspring_two, None); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_one.mutate(num_variables); }
            if rand::thread_rng().gen_bool(mutation_chance) { offspring_two.mutate(num_variables); }

            new_population.push(offspring_one);
            new_population.push(offspring_two);
        }

        return new_population;
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
        return vec![];
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
        return if (c1.fitness_value < c2.fitness_value) { c1 } else { c2 };
    }

    /// Returns a reference to a randomly selected `Chromosome` from the `self` vector.
    fn get_random_chromosome(&self) -> &Chromosome {
        return &self[rand::thread_rng().gen_range(0..self.len())];
    }
}