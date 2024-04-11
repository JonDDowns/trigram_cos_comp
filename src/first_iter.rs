use itertools::Itertools;
use rayon::prelude::*;
use textdistance::{Algorithm, Cosine};

// This iteration uses the out-of-box function from text distance.
// This was the most inefficient method, as behind the hood HashMap
// is recalculating the same hashmap many times

pub fn cos_tri(inputs: Vec<String>) -> Vec<f64> {
    println!("Running v1");
    let allchar: Vec<Vec<(char, char, char)>> = inputs
        .iter()
        .map(|x| {
            x.chars()
                .tuple_windows::<(char, char, char)>()
                .collect_vec()
        })
        .collect();

    let alg: Cosine = Cosine::default();
    let result: Vec<f64> = allchar
        .par_iter()
        .enumerate()
        .map(|(i, a)| {
            allchar[(i + 1)..]
                .iter()
                .map(|x| alg.for_vec(a, x).nval())
                .collect::<Vec<f64>>()
        })
        .flatten()
        .collect();
    result
}

