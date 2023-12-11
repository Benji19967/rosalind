use std::io::{self, Read};

use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut strings: Vec<&str> = vec![];
    for line in input.lines() {
        strings.push(line);
    }
    let d = find_substrings(strings[0], strings[1]);
    println!("{:?}", d.iter().map(|e| e.to_string()).join(" "));

    Ok(())
}

fn find_substrings(s: &str, t: &str) -> Vec<u32> {
    let mut locations: Vec<u32> = vec![];
    for i in 0..(s.len() - t.len() + 1) {
        if &s[i..i + t.len()] == t {
            locations.push((i + 1) as u32);
        }
    }
    locations
}

#[cfg(test)]
mod tests {
    use crate::find_substrings;

    #[test]
    fn subs1() {
        assert_eq!(find_substrings("ATATA", "TA"), vec![2, 4]);
    }
}
