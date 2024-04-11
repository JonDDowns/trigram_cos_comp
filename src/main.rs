pub mod final_iter;
pub mod first_iter;
pub mod second_iter;
use std::env;
use std::fs::read_to_string;

// Reads in the example file
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn main() {
    // Parse environment args and read in data
    let inargs: Vec<String> = env::args().collect();
    let inputs: Vec<String> = read_lines(&inargs[1]);
    println!("Number of unique words: {:?}", inputs.len());

    // Dispatch based on CLI args
    let result: Vec<f64> = if inargs.len() > 2 {
        match inargs[2].as_ref() {
            "v1" => first_iter::cos_tri(inputs),
            "v2" => second_iter::cos_tri(inputs),
            _ => final_iter::cos_tri(inputs),
        }
    } else {
        final_iter::cos_tri(inputs)
    };

    println!("Number of calculated distances: {:?}", result.len());

    // Check that sum of all results adds up to same value in each method
    let total = result.iter().sum::<f64>().round();
    assert_eq!(total, 4448996.0);
}
