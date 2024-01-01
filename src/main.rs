// #![allow(arithmetic_overflow)]
use std::any::Any;

use clap::Parser;
use rand::Rng;

use crate::chromosome::Chromosome;
use crate::read_dataset::read_csv;

mod chromosome;
mod functions;
mod read_dataset;

fn main() {
    // let matches = Command::new("Rust GP")
    //     .version("0.1.0")
    //     .author("")
    //     .about("Genetic Program that uses an acyclic graph representation and (probably) fitness predictors")
    //     .arg(Arg::new("file")
    //         .short('f')
    //         .long("file")
    //         .help("A CSV file containing the values you are trying to regress toward with symbolic regression"))
    //     .arg(Arg::new("generations")
    //         .short('g')
    //         .long("generations")
    //         .help("The number of generations for the GP"))
    //     .arg(Arg::new("population")
    //         .short('p')
    //         .long("population")
    //         .help("The population size for the GP"))
    //     .arg(Arg::new("crossover chance")
    //         .short('c')
    //         .long("crossover_chance")
    //         .help(""))
    //     .arg(Arg::new("mutation chance")
    //         .short('m')
    //         .long("mutation_chance")
    //         .help(""))
    //     .get_matches();


    // let c1 = Chromosome::new_x(5, 3);
    // let x = Vec::from([1.0, 1.0, 1.0]);
    // println!("{}", c1.to_string());
    // println!("{}", c1.make_function_string(None, "".to_string()));
    // println!("{}", c1.evaluate_fitness(&x));
    let dataset = read_csv("E:\\Code\\rust_gp\\VOLUNTEER1_trial_1_duplicate_task_na_control.csv");
    gp(100, 100, 40, 0.5, 0.5, dataset);
    // println!("{}", (f64::MAX * 2.5))
}

pub fn gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Vec<Vec<f64>>) {
    let mut population = Population::new();

    for _p in 0..pop_size {
        population.push(Chromosome::new_x(num_genes, dataset[0].len() - 2))
    }

    for g in 0..gen {
        for mut i in &mut population {
            i.evaluate_fitness_mse(&dataset);
        }
        population = population.mate(dataset[0].len() - 2);
    }

    println!("{}", population.find_best_min().evaluate_fitness_mse(&dataset))
}

fn min_selection(population: &Vec<Chromosome>) -> &Chromosome {
    // Selects a random chromosome from the population
    let select_random = || -> &Chromosome { &population[rand::thread_rng().gen_range(0..population.len())] };
    let c1 = select_random();
    let c2 = select_random();
    return if (c1.fitness_value > c2.fitness_value) { c1 } else { c2 };
}
// pub fn debug_new() -> [Gene; 3] {
//     pub fn add(x: f64, y: f64) -> f64 {
//         x + y
//     }
//
//     pub fn square(x: f64, _y: f64) -> f64 {
//         x * x
//     }
//
//     let mut genes = [
//         Gene {
//             type_of_gene: chromosome::GeneType::Binary,
//             left_ptr: 1,
//             right_ptr: 2,
//             ops: add,
//         },
//         Gene {
//             type_of_gene: chromosome::GeneType::Variable(1),
//             left_ptr: 5,
//             right_ptr: usize::MAX,
//             ops: add,
//         },
//         Gene {
//             type_of_gene: chromosome::GeneType::Constant(5.0),
//             left_ptr: 5,
//             right_ptr: usize::MAX,
//             ops: add,
//         },
//     ];
//
//     genes
// }