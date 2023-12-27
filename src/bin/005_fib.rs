use std::io::{self, Read};

// Input: 5 3, where n=5 and k=3
// Output: 19
// Recurrence relation: F_n = F_n-1 + k*F_n-2
// F_0 = 0, F_1 = 1

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    for line in input.lines() {
        let split: Vec<u64> = line
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect();
        let n = split[0];
        let k = split[1];
        let num_rabbits = fib(n as usize, k);
        println!("{}", num_rabbits);
    }

    Ok(())
}

fn fib(n: usize, k: u64) -> u64 {
    let mut dp: Vec<u64> = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i as usize] = dp[(i - 1) as usize] + k * dp[(i - 2) as usize];
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn fib1() {
        assert_eq!(fib(5, 3), 19);
    }
    #[test]
    fn fib2() {
        assert_eq!(fib(5, 1), 5);
    }
}
