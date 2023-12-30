use rand::Rng;

pub fn get_unary_function() -> fn(f64, f64) -> (f64, String) {
    let unary_functions: Vec<fn(f64, f64) -> (f64, String)> = vec![square, log2];
    let random_string_index: usize = rand::thread_rng().gen_range(0..unary_functions.len());
    unary_functions[random_string_index]
}

pub fn get_binary_function() -> fn(f64, f64) -> (f64, String) {
    let binary_functions: Vec<fn(f64, f64) -> (f64, String)> = vec![add];
    let random_string_index: usize = rand::thread_rng().gen_range(0..binary_functions.len());
    binary_functions[random_string_index]
}

pub fn add(x: f64, y: f64) -> (f64, String) {
    (x + y, "add".to_string())
}

pub fn square(x: f64, _y: f64) -> (f64, String) {
    (x * x, "square".to_string())
}

pub fn log2(x: f64, _y: f64) -> (f64, String) {
    (x.log2(), "log2".to_string())
}