use std::time::Instant;

use crate::io::{Dataset, DataToWrite};
use crate::population::{Population, PopulationTraits};

pub fn gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Dataset) {
    let now = Instant::now();
    let mut population = Population::initialize(pop_size, num_genes, &dataset);
    let mut fitness_graph: Vec<DataToWrite> = vec![];


    for g in 0..gen {
        population.evaluate(&dataset);

        let best = population.mate(dataset[0].len() - 2, crossover_chance, mut_chance, &dataset);
        fitness_graph.push(DataToWrite { generation: g, fitness: best });
    }


    println!("{}", population.best.evaluate_fitness_mse(&dataset));
    println!("{}", population.best.make_function_string(None, "".parse().unwrap()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    crate::io::write_graph_data(fitness_graph, "gp_out.txt")
}