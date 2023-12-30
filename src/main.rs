use crate::chromosome::Chromosome;

mod chromosome;
mod functions;
mod read_dataset;

fn main() {
    let c1 = Chromosome::new_x(500, 3);
    let x = Vec::from([1.0, 1.0, 1.0]);
    println!("{}", c1.to_string());
    println!("{}", c1.make_function_string(None, "".to_string()));
    println!("{}", c1.evaluate_fitness(&x));
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