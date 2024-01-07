use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use csv::ReaderBuilder;

pub fn read_csv(location: &str) -> Vec<Vec<f64>> {
    let mut csv = Vec::new();
    let rdr = ReaderBuilder::new().from_path(location);

    for r in rdr.unwrap().records() {
        let record = r.unwrap();
        let mut temp_csv = Vec::new();
        record.iter().for_each(|x| temp_csv.push(x.parse::<f64>().unwrap()));
        csv.push(temp_csv)
    }
    return csv;
}

type Dataset = Vec<Vec<f64>>;

pub struct DataToWrite {
    pub(crate) generation: usize,
    pub(crate) fitness: f64,
}

impl Display for DataToWrite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}, {}", self.generation, self.fitness)
    }
}

pub fn write_graph_data(data: Vec<DataToWrite>, file_name: &str) {
    let file = File::create(file_name).unwrap();
    let mut file = BufWriter::new(file);
    for row in data {
        writeln!(file, "{}, {}", row.generation, row.fitness).expect("Problem writing to file")
    }
}