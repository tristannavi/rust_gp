use std::fs;
use csv::ReaderBuilder;

pub fn read_csv(location: &str) -> Vec<Vec<f64>> {
    let mut csv = Vec::new();
    let data = "\
city;country;pop
5.0;1.0;4.0
";
    let mut rdr = ReaderBuilder::new().from_path("E:\\Code\\rust_gp\\VOLUNTEER1_trial_1_duplicate_task_na_control.csv");

    for r in rdr.unwrap().records() {
        let record = r.unwrap();
        let mut temp_csv = Vec::new();
        record.iter().for_each(|x| temp_csv.push(x.parse::<f64>().unwrap()));
        csv.push(temp_csv)
    }
    return csv;
}

type Dataset = Vec<Vec<f64>>;

trait DatasetFunctions {
}