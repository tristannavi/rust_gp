//Seems slower when mut is not used
#![allow(unused_mut)]

use std::thread;
use std::time::Instant;

use crate::io::{Dataset, DataToWrite};
use crate::island::{Archipelago, ArchipelagoTraits, IslandParameters};
use crate::population::{Population, PopulationParameters, PopulationTraits};

pub fn gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Vec<Vec<f64>>) {
    let now = Instant::now();
    let mut population = Population::initialize(pop_size, num_genes, &dataset);
    let mut fitness_graph: Vec<DataToWrite> = vec![];


    for g in 0..gen {
        population.evaluate(&dataset);

        let best;
        (population, best) = population.mate(dataset[0].len() - 2, crossover_chance, mut_chance);
        fitness_graph.push(DataToWrite { generation: g, fitness: best });

        //INFO: Ensures that all threads have finished before getting here (all chromosomes were evaluated)
        population.all_accessed();
    }


    println!("{}", population.find_best_min().evaluate_fitness_mse(&dataset));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    crate::io::write_graph_data(fitness_graph, "gp_out.txt")
}

pub fn island_gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Vec<Vec<f64>>) {
    let now = Instant::now();
    let island_parameters = IslandParameters {
        population_parameters: PopulationParameters {
            generations: gen,
            population_size: pop_size,
            num_genes,
            mut_chance,
            crossover_chance,
            dataset,
        },
        num_islands: 2,
        migration_count: 0,
        mutation_number: 0,
    };
    let mut archipelago = Archipelago::initialize(&island_parameters);

    for g in 0..gen {
        thread::scope(|s| {
            for mut island in &mut archipelago {
                s.spawn(|| {
                    island.evaluate_fitness(&island_parameters.population_parameters.dataset);
                });
            };
        });
        for a in 0..archipelago.len() {
            // archipelago[a] = archipelago[a].mate(&island_parameters.population_parameters.dataset[0].len() - 2, crossover_chance, mut_chance);
        }
    }

//INFO: Ensures that all threads have finished before getting here
//     let mut temp = 0;
//     for x in &population {
//         if !x.accessed { temp += 1 }
//     }
//     assert_eq!(temp, 0);
//
//     println!("{}", population.find_best_min().evaluate_fitness_mse(&dataset));
//     let elapsed = now.elapsed();
//     println!("Elapsed: {:.2?}", elapsed);
}
