use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

// In the final iteration, we repeat the second run with a faster hashing algorithm

// Generates the count for each string
fn make_ctr(input: &String) -> FxHashMap<(char, char, char), usize> {
    let win = input.chars().tuple_windows::<(char, char, char)>();
    let mut ctr = FxHashMap::default();
    for item in win {
        let entry = ctr.entry(item).or_insert(0);
        *entry += 1;
    }
    ctr
}

// Calculates cosine similarity for all pairwise combinations of a given list
pub fn cos_tri(inputs: Vec<String>) -> Vec<f64> {
    println!("Running Final Iteration");
    let charcts: Vec<FxHashMap<(char, char, char), usize>> =
        inputs.iter().map(|x| make_ctr(x)).collect();

    let result: Vec<f64> = charcts
        .par_iter()
        .enumerate()
        .map(|(i, c1)| {
            charcts[(i + 1)..]
                .iter()
                .map(|c2| {
                    let n1 = c1.values().sum();
                    let n2 = c2.values().sum();
                    let res = match (n1, n2) {
                        (0, 0) => 1.,
                        (_, 0) | (0, _) => 0.,
                        (_, _) => {
                            let mut result = 0;
                            for (key, lhs_count) in c1 {
                                if let Some(rhs_count) = c2.get(key) {
                                    result += lhs_count.min(rhs_count);
                                }
                            }
                            result as f64 / ((n1 * n2) as f64).sqrt()
                        }
                    };
                    res
                })
                .collect::<Vec<f64>>()
        })
        .flatten()
        .collect();
    result
}
