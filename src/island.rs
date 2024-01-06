use crate::chromosome::Chromosome;
use crate::population::{Island, Population, PopulationParameters};

// gen: usize, pop_size: usize, num_genes: usize, mut_chance: f64
// , crossover_chance: f64, dataset: Vec<Vec<f64>>
pub struct IslandParameters {
    pub population_parameters: PopulationParameters,
    pub num_islands: usize,
    pub migration_count: usize,
    pub mutation_number: usize,
}

pub type Archipelago = Vec<Island>;

pub trait ArchipelagoTraits {
    fn new_n(n: usize) -> Archipelago;
    fn initialize(n: &IslandParameters) -> Archipelago;
}

impl ArchipelagoTraits for Archipelago {
    fn new_n(n: usize) -> Archipelago {
        let mut archipelago = vec![];
        for x in 0..n {
            archipelago.push(Population::new());
            for a in &archipelago[x] {}
        }

        return archipelago;
    }

    fn initialize(p: &IslandParameters) -> Archipelago {
        let mut archipelago = vec![] as Archipelago;
        for island in 0..p.num_islands {
            archipelago.push(Population::new());
            for individual in 0..p.population_parameters.population_size {
                archipelago[individual].push(Chromosome::new_x(
                    p.population_parameters.num_genes,
                    p.population_parameters.dataset.len() - 2,
                ))
            }
        }

        return archipelago;
    }
}