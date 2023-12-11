use std::{
    io::{self, Read},
    iter::zip,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut strings: Vec<&str> = vec![];
    for line in input.lines() {
        strings.push(line);
    }
    let d = hamming_distance(strings[0], strings[1]);
    println!("{}", d);

    Ok(())
}

fn hamming_distance(s: &str, t: &str) -> u32 {
    let mut distance = 0;
    for (c1, c2) in zip(s.chars(), t.chars()) {
        if c1 != c2 {
            distance += 1;
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use crate::hamming_distance;

    #[test]
    fn hamm1() {
        assert_eq!(hamming_distance("A", "G"), 1);
    }
    #[test]
    fn hamm2() {
        assert_eq!(hamming_distance("AC", "AG"), 1);
    }
    #[test]
    fn hamm3() {
        assert_eq!(hamming_distance("ACAT", "AGTA"), 3);
    }
}
