use std::any::Any;
use std::thread;

use clap::{Arg, Command, Parser, value_parser};
use rand::Rng;

use crate::chromosome::Chromosome;
use crate::population::{Mate, Population};
use crate::read_dataset::read_csv;
use crate::io::read_csv;

mod chromosome;
mod functions;
mod io;
mod population;
mod island;
mod gp;

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

    if *matches.get_one::<usize>("population").unwrap() % 2 == 0 {
        panic!("The number of individuals in the population must be odd for elitism to work")
    }

    let dataset = read_csv(matches.get_one::<String>("file").unwrap());
    gp::gp(
        *matches.get_one::<usize>("generations").unwrap(),
        *matches.get_one::<usize>("population").unwrap(),
        *matches.get_one::<usize>("num genes").unwrap(),
        *matches.get_one::<f64>("mutation chance").unwrap(),
        *matches.get_one::<f64>("crossover chance").unwrap(),
        dataset,
    );
}
