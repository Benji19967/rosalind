use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut strings: Vec<&str> = vec![];
    for line in input.lines() {
        strings.push(line);
    }
    let profile_matrix = build_profile_matrix();
    let consensus_string = build_consensus_string();

    Ok(())
}

fn build_profile_matrix() -> Vec<Vec<u32>> {
    unimplemented!()
}

fn build_consensus_string() -> Vec<Vec<u32>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::build_consensus_string;
    use crate::build_profile_matrix;

    // #[test]
    // fn cons1() {
    //     assert_eq!(create_profile_matrix(), vec![2, 4]);
    // }
}
