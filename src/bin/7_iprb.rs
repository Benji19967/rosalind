use bio::stats::combinatorics::combinations;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    for line in input.lines() {
        let split: Vec<u64> = line
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect();
        let (k, m, n) = (split[0], split[1], split[2]);
        let d = probability_dominant(k, m, n);
        println!("{}", d);
    }

    Ok(())
}

/// The probability for pairs is computed using a Punnett square.
///
/// For example, m_n:
///
///      Y  y
/// ---------
/// y | yY yy
/// y | yY yy
///
/// --> 2 of 4 are dominant, so the probablity that a pair of m and n
/// produce an individual possessing a dominant allel is 0.5
///
fn probability_dominant(k: u64, m: u64, n: u64) -> f64 {
    let total_population = k + m + n;
    let total_combinations = combinations(total_population, 2);

    // Total number of possible pairs for each pair of organisms
    let k_k = combinations(k, 2);
    let m_m = combinations(m, 2);
    let n_n = combinations(n, 2);
    let k_m = (k * m) as f64;
    let k_n = (k * n) as f64;
    let m_n = (m * n) as f64;

    let p = (k_k * 1.0 + m_m * 0.75 + n_n * 0.0 + k_m * 1.0 + k_n * 1.0 + m_n * 0.5)
        / total_combinations;

    println!("{}", p);
    p
}

#[cfg(test)]
mod tests {
    use crate::probability_dominant;

    #[test]
    fn fib1() {
        assert_eq!(probability_dominant(2, 2, 2), 0.78333);
    }
}
