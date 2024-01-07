//Seems slower when mut is not used
#![allow(unused_mut)]

use std::time::Instant;

use crate::io::{Dataset, DataToWrite};
use crate::population::{Population, PopulationTraits};

pub fn gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Dataset) {
    let now = Instant::now();
    let mut population = Population::initialize(pop_size, num_genes, &dataset);
    let mut fitness_graph: Vec<DataToWrite> = vec![];


    for g in 0..gen {
        population.evaluate(&dataset);

        let best = population.mate(dataset[0].len() - 2, crossover_chance, mut_chance);
        fitness_graph.push(DataToWrite { generation: g, fitness: best });
    }


    println!("{}", population.find_best_min().evaluate_fitness_mse(&dataset));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    crate::io::write_graph_data(fitness_graph, "gp_out.txt")
}