use std::fmt::{Display, Formatter};
use std::mem::swap;

use rand::{random, Rng};
use rand::seq::SliceRandom;

use crate::chromosome::GeneType::{Binary, Constant, Unary, Variable};
use crate::functions;

#[derive(Debug)]
pub enum GeneType {
    Constant(f64),
    Variable(usize),
    Unary,
    Binary,
}

impl Display for GeneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant(i) => write!(f, "Constant({})", i),
            Variable(i) => write!(f, "Variable({})", i),
            Unary => write!(f, "Unary"),
            Binary => write!(f, "Binary")
        }
    }
}

impl Clone for GeneType {
    fn clone(&self) -> Self {
        return match self {
            Constant(i) => { Constant(i.clone()) }
            Variable(i) => { Variable(i.clone()) }
            Unary => { Unary }
            Binary => { Binary }
        };
    }
}

impl Clone for Gene {
    fn clone(&self) -> Self {
        return Gene {
            type_of_gene: self.type_of_gene.clone(),
            left_ptr: self.left_ptr,
            right_ptr: self.right_ptr,
            ops: self.ops,
        };
    }
}

impl Display for Gene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.type_of_gene {
            Constant(i) => write!(f, "Constant({})[{}, {}]", i, self.left_ptr, self.right_ptr),
            Variable(i) => write!(f, "Variable({})[{}, {}]", i, self.left_ptr, self.right_ptr),
            Unary => write!(f, "Unary[{}, {}]", self.left_ptr, self.right_ptr),
            Binary => write!(f, "Binary[{}, {}]", self.left_ptr, self.right_ptr)
        }
    }
}

pub struct Gene {
    pub type_of_gene: GeneType,
    pub left_ptr: usize,
    pub right_ptr: usize,
    pub ops: fn(f64, f64) -> (f64, String),
}


impl Gene {
    pub fn new(curr_loc: usize, num_variables: usize, first_or_second_in_chromosome: bool) -> Gene {
        return if random() || first_or_second_in_chromosome {
            if random() { Gene::new_constant(None) } else { Gene::new_variable(num_variables) }
        } else {
            if random() { Gene::new_binary(curr_loc) } else { Gene::new_unary(curr_loc) }
        };
    }

    fn new_constant(constant: Option<f64>) -> Gene {
        return Gene {
            type_of_gene: Constant(constant.unwrap_or(random())),
            left_ptr: 0,
            right_ptr: 0,
            ops: Gene::nothing,
        };
    }

    fn new_variable(num_variables: usize) -> Gene {
        return Gene {
            type_of_gene: Variable(rand::thread_rng().gen_range(0..num_variables - 1)),
            left_ptr: 0,
            right_ptr: 0,
            ops: Gene::nothing,
        };
    }

    /// Constructs a new unary gene.
    ///
    /// # Arguments
    ///
    /// * `curr_loc` - The current location in the genome (`Chromosome`).
    ///
    /// # Returns
    ///
    /// A `Gene` struct representing the unary gene.
    fn new_unary(curr_loc: usize) -> Gene {
        return Gene {
            type_of_gene: Unary,
            left_ptr: rand::thread_rng().gen_range(0..curr_loc),
            right_ptr: 0,
            ops: functions::get_unary_function(),
        };
    }

    /// This function creates a new binary Gene.
    ///
    /// # Arguments
    ///
    /// * `curr_loc` - The location (index) of the `Gene` within the `Chromosome`.
    ///
    /// # Returns
    ///
    /// A `Gene` struct with the following fields:
    /// * `type_of_gene` - The type of gene, which is set to `Binary`.
    /// * `left_ptr` - A randomly generated value between 0 and `curr_loc`, representing the left pointer.
    /// * `right_ptr` - A randomly generated value between 0 and `curr_loc`, representing the right pointer.
    /// * `ops` - The binary function retrieved from the `get_binary_function` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Gene;
    ///
    /// let curr_loc = 10;
    /// let gene = new_binary(curr_loc);
    /// ```
    fn new_binary(curr_loc: usize) -> Gene {
        return Gene {
            type_of_gene: Binary,
            left_ptr: rand::thread_rng().gen_range(0..curr_loc),
            right_ptr: rand::thread_rng().gen_range(0..curr_loc),
            ops: functions::get_binary_function(),
        };
    }

    /// Calculates and returns the result of doing nothing. Used as a placeholder for genes that do not do anything with the provided function.
    ///
    /// # Arguments
    ///
    /// * `_x` - The first input parameter of type `f64`. It is ignored.
    /// * `_y` - The second input parameter of type `f64`. It is ignored.
    ///
    /// # Returns
    ///
    /// The result of doing nothing, which is always `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = nothing(5.0, 10.0);
    /// assert_eq!(result, 0.0);
    /// ```
    fn nothing(_x: f64, _y: f64) -> (f64, String) {
        (0.0, "nothing".to_string())
    }

    /// Performs the operation on a gene using it's left and right pointers
    ///
    /// # Arguments
    ///
    /// * `chromosome`: The chromosome containing the genes
    /// * `vec`: One row of the dataset
    ///
    /// returns: `f64`
    pub fn operation(&self, chromosome: &Chromosome, vec: &Vec<f64>) -> f64 {
        return match self.type_of_gene {
            Constant(x) => x,
            Unary => (self.ops)(chromosome.genes[self.left_ptr].operation(chromosome, vec), -1.0).0,
            Binary => (self.ops)(chromosome.genes[self.left_ptr].operation(chromosome, vec), chromosome.genes[self.right_ptr].operation(chromosome, vec)).0,
            Variable(x) => vec[x],
        };
    }

    /// Returns the type of the function.
    pub fn get_type(&self) -> String {
        return ((&self).ops)(0.0, 0.0).1;
    }
}

/// Represents a chromosome with genes and fitness value.
#[derive(Clone)]
pub struct Chromosome {
    pub genes: Vec<Gene>,
    pub fitness_value: f64,
    pub accessed: bool,
}

impl Chromosome {
    /// Creates a new `Chromosome` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let chromosome = Chromosome::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Chromosome` instance with an empty gene vector and maximum fitness value.
    pub fn new() -> Chromosome {
        Chromosome {
            genes: Vec::new(),
            fitness_value: f64::MAX,
            accessed: false,
        }
    }

    /// Creates a new `Chromosome` from a given `genes_array`.
    ///
    /// # Arguments
    ///
    /// * `genes_array` - A `Vec<Gene>` containing the genes to be assigned to the `Chromosome`.
    ///
    /// # Returns
    ///
    /// A new `Chromosome` instance with the given genes and the maximum fitness value.
    pub fn new_from_genes_array(genes_array: Vec<Gene>) -> Chromosome {
        Chromosome {
            genes: genes_array,
            fitness_value: f64::MAX,
            accessed: false, // Thread testing
        }
    }

    /// Generates a new Chromosome with x number of genes. Each Gene is randomly generated.
    ///
    /// The first and second genes will always be a constant or a variable
    ///
    /// # Arguments
    ///
    /// * `num_genes`: How many genes to generate
    /// * `num_variables`: How many variables there are in the dataset
    ///
    /// returns: Chromosome
    ///
    /// # Examples
    ///
    /// ```
    /// let c = Chromosome::new_x(5, 5)
    /// ```
    pub fn new_x(num_genes: usize, num_variables: usize) -> Chromosome {
        let mut c = Chromosome::new();
        for i in 0..num_genes {
            c.genes.push(Gene::new(i, num_variables, i == 1 || i == 0))
        }
        return c;
    }

    /// Evaluates the fitness of an individual based on a given vector of values.
    ///
    /// # Arguments
    ///
    /// * `vec` - A reference to a vector of floating-point values (one row of the values that the GP is using)
    ///
    /// # Returns
    ///
    /// * The fitness value as a `f64` number.
    pub fn evaluate_fitness(&self, vec: &Vec<f64>) -> f64 {
        // self.fitness_value = self.genes[self.genes.len() - 1].operation(&self, vec);
        return self.genes[self.genes.len() - 1].operation(&self, vec);
    }

    /// Calculates the mean squared error (MSE) fitness of the given dataset for a `Chromosome`.
    ///
    /// The MSE fitness is a measure of how well the genetic algorithm's prediction matches the expected output.
    /// It is calculated by summing the squared differences between the predicted and expected values for each row in the dataset,
    /// and then dividing the sum by the number of rows in the dataset.
    ///
    /// # Arguments
    ///
    /// * `vec` - A reference to a vector of vectors representing the dataset. Each sub-vector represents a row in the dataset,
    ///           with the last element in the row being the expected output.
    ///
    /// # Returns
    ///
    /// Returns the calculated mean squared error as a `f64` value. If the value calculated is infinity, it will return `f64::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::GeneticAlgorithm;
    ///
    /// let c = Chromosome::new_x(5); // Create chromosome with 5 genes
    /// let dataset = vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![4.0, 5.0, 6.0],
    ///     vec![7.0, 8.0, 9.0]
    /// ];
    /// let mse = c.evaluate_fitness_mse(&dataset);
    /// ```
    pub fn evaluate_fitness_mse(&mut self, vec: &Vec<Vec<f64>>) -> f64 {
        let mut total: f64 = 0.0;
        for row in vec {
            let expected = row[row.len() - 1];
            let predicted = self.evaluate_fitness(row);
            total += (predicted - expected).powi(2);
        }
        return match total.is_infinite() {
            true => {
                self.accessed = true; // Thread testing
                self.fitness_value = f64::MAX;
                f64::MAX
            }
            false => {
                self.accessed = true; // Thread testing
                self.fitness_value = total;
                total / (vec.len() as f64)
            }
        };
    }

    fn iter(&self) -> impl Iterator<Item=&Gene> {
        self.genes.iter()
    }

    /// Returns the length of the genes array (`Chromosome`) in the provided instance.
    ///
    /// # Example
    /// ```
    /// let instance = Instance { genes: vec![1, 2, 3] };
    /// assert_eq!(instance.len(), 3);
    /// ```
    ///
    /// # Returns
    ///
    /// - `usize`: The length of the genes array (`Chromosome`) in the provided instance.
    fn len(&self) -> usize {
        self.genes.len()
    }

    /// Converts a GeneticExpression into a string representation of the function.
    ///
    /// # Arguments
    /// * `position` - An optional position parameter. If `Some`, the function will start the conversion from this position in the gene list. If `None`, it will start from the last gene in the list.
    pub fn make_function_string(&self, position: Option<usize>, mut builder: String) -> String {
        let pos = position.unwrap_or(self.genes.len() - 1);
        match &self.genes[pos].type_of_gene {
            Constant(i) => {
                builder.push_str(&format!("{}", i));
            }
            Variable(i) => {
                builder.push_str(&format!("v{}", i));
            }
            Unary => {
                builder.push_str(&format!("{}({})", &self.genes[pos].get_type(), &self.make_function_string(Some(self.genes[pos].left_ptr), builder.clone())))
            }
            Binary => {
                builder.push_str(&format!("{}({}, {})", &self.genes[pos].get_type(), &self.make_function_string(Some(self.genes[pos].left_ptr), builder.clone()), &self.make_function_string(Some(self.genes[pos].right_ptr), builder.clone())))
            }
        }
        return builder.to_string();
    }

    /// Shuffles the genes within the struct.
    ///
    /// This function shuffles the genes within the struct using the Fisher-Yates algorithm.
    /// It modifies the genes in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::seq::SliceRandom;
    ///
    /// // Create a new instance of the struct
    /// let mut c = Chromosome::new();
    ///
    /// // Shuffle the genes within the struct
    /// c.shuffle();
    ///
    /// // Print the shuffled genes
    /// println!("{:?}", my_struct.genes);
    /// ```
    pub fn shuffle(&mut self) {
        &self.genes.shuffle(&mut rand::thread_rng());
    }
}

impl Chromosome {
    /// Crosses the current chromosome with another chromosome.
    ///
    /// # Arguments
    ///
    /// * `parent_2` - A mutable reference to the second parent chromosome.
    /// * `crossover_loc` - Optional. The index at which the crossover operation will start.
    ///                    If not provided, a random index between 0 and the length of the current chromosome is chosen.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chromosome_1 = Chromosome::new();
    /// let mut chromosome_2 = Chromosome::new();
    ///
    /// chromosome_1.cross_with(&mut chromosome_2, None);
    /// ```
    pub fn cross_with(&mut self, parent_2: &mut Chromosome, crossover_loc: Option<usize>) {
        let cross_loc = crossover_loc.unwrap_or(rand::thread_rng().gen_range(0..self.len()));
        for i in cross_loc..self.len() {
            swap(&mut self.genes[i], &mut parent_2.genes[i])
        }
    }

    /// Mutates a gene by randomly selecting a location within the gene and replacing it with a new random gene.
    ///
    /// # Arguments
    ///
    /// * `num_variables` - The number of variables in the GP dataset.
    ///
    /// # Example
    ///
    /// ```
    /// let c = Chromosome::New()
    /// c.mutate(5)
    /// ```
    pub fn mutate(&mut self, num_variables: usize) {
        let mut_loc = rand::thread_rng().gen_range(0..self.len());
        self.genes[mut_loc] = Gene::new(mut_loc, num_variables, (mut_loc == 0) || (mut_loc == 1))
    }
}

impl Display for Chromosome {
    ///
    /// Formats the genes in a string and writes them to the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `std::fmt::Formatter` object.
    ///
    /// # Errors
    ///
    /// This function returns a `std::fmt::Result` object. It will return an
    /// `Err` value if writing the formatted string to the formatter fails.
    ///
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string_builder = "".to_string();
        for gene in &self.genes {
            string_builder.push_str(&*gene.to_string());
            string_builder.push_str(" ");
        }
        write!(f, "{}", string_builder)
    }
}