// #![allow(arithmetic_overflow)]
use std::any::Any;
use std::thread;

use clap::{Arg, Command, Parser, value_parser};
use rand::Rng;

use crate::chromosome::Chromosome;
use crate::read_dataset::read_csv;

mod chromosome;
mod functions;
mod read_dataset;

fn main() {
    let matches = Command::new("Rust GP")
        .version("0.1.0")
        .author("")
        .about("Genetic Program that uses an acyclic graph representation and (probably) fitness predictors")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("A CSV file containing the values you are trying to regress toward with symbolic regression")
            .default_value("E:\\Code\\rust_gp\\VOLUNTEER1_trial_1_duplicate_task_na_control.csv")
            .value_parser(value_parser!(String)))
        .arg(Arg::new("num genes")
            .short('n')
            .long("genes")
            .help("The number of genes in the Chromosome")
            .default_value("100")
            .value_parser(value_parser!(usize)))
        .arg(Arg::new("generations")
            .short('g')
            .long("generations")
            .help("The number of generations for the GP")
            .default_value("100")
            .value_parser(value_parser!(usize)))
        .arg(Arg::new("population")
            .short('p')
            .long("population")
            .help("The population size for the GP")
            .default_value("101")
            .value_parser(value_parser!(usize)))
        .arg(Arg::new("crossover chance")
            .short('c')
            .long("crossover_chance")
            .help("")
            .default_value("0.5")
            .value_parser(value_parser!(f64)))
        .arg(Arg::new("mutation chance")
            .short('m')
            .long("mutation_chance")
            .help("")
            .default_value("0.5")
            .value_parser(value_parser!(f64)))
        .get_matches();

    if (*matches.get_one::<usize>("population").unwrap() % 2 == 0) {
        panic!("The number of individuals in the population must be odd for elitism to work")
    }

    let dataset = read_csv(matches.get_one::<String>("file").unwrap());
    gp(
        *matches.get_one::<usize>("generations").unwrap(),
        *matches.get_one::<usize>("population").unwrap(),
        *matches.get_one::<usize>("num genes").unwrap(),
        *matches.get_one::<f64>("mutation chance").unwrap(),
        *matches.get_one::<f64>("crossover chance").unwrap(),
        dataset,
    );
}

pub fn gp(gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64, crossover_chance: f64, dataset: Vec<Vec<f64>>) {
    use std::time::Instant;
    let now = Instant::now();
    let mut population = Population::new();

    for _p in 0..pop_size {
        population.push(Chromosome::new_x(num_genes, dataset[0].len() - 2))
    }

    for g in 0..gen {
        thread::scope(|s| {
            for mut i in &mut population {
                s.spawn(|| {
                    let temp1 = i.evaluate_fitness_mse(&dataset);
                });
            }
        });
        population = population.mate(dataset[0].len() - 2, crossover_chance, mut_chance);
    }

    //INFO: Ensures that all threads have finished before getting here
    let mut temp = 0;
    for x in &population {
        if !x.accessed { temp += 1 }
    }
    assert_eq!(temp, 0);

    println!("{}", population.find_best_min().evaluate_fitness_mse(&dataset));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
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