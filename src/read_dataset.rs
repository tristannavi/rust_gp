use csv::ReaderBuilder;

pub fn read_csv() -> Vec<Vec<f64>> {
    let mut csv = Vec::new();
    let data = "\
city;country;pop
5.0;1.0;4.0
";
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    for r in rdr.records() {
        let record = r.unwrap();
        let mut temp_csv = Vec::new();
        record.iter().for_each(|x| temp_csv.push(x.parse::<f64>().unwrap()));
        csv.push(temp_csv)
    }
    return csv;
}