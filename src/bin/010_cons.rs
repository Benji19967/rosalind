use rosalind::utils;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let dna_strings = utils::read_fasta(input);
    let profile_matrix = build_profile_matrix(&dna_strings.values().collect()).unwrap();
    let consensus_string = build_consensus_string(&profile_matrix);
    println!("{:?}", consensus_string);
    display_profile_matrix(&profile_matrix);

    Ok(())
}

fn build_profile_matrix(dna_strings: &Vec<&String>) -> Result<Vec<Vec<u32>>> {
    let len_dna_strings: usize = dna_strings[0].len();
    let mut profile_matrix: Vec<Vec<u32>> = vec![vec![0; len_dna_strings]; 4];
    for dna_string in dna_strings {
        for (i, c) in dna_string.chars().enumerate() {
            match c {
                'A' => {
                    profile_matrix[0][i] += 1;
                    Ok(())
                }
                'C' => {
                    profile_matrix[1][i] += 1;
                    Ok(())
                }
                'G' => {
                    profile_matrix[2][i] += 1;
                    Ok(())
                }
                'T' => {
                    profile_matrix[3][i] += 1;
                    Ok(())
                }
                _ => Err("Found invalid character in DNA string"),
            }?;
        }
    }
    Ok(profile_matrix)
}

fn build_consensus_string(profile_matrix: &[Vec<u32>]) -> String {
    let len_dna_strings = profile_matrix[0].len();
    let mut max_occurence: Vec<u32> = vec![0; len_dna_strings];
    let mut consensus_string: Vec<char> = vec!['A'; len_dna_strings];

    for (row_idx, row) in profile_matrix.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if profile_matrix[row_idx][col_idx] > max_occurence[col_idx] {
                max_occurence[col_idx] = profile_matrix[row_idx][col_idx];
                consensus_string[col_idx] = row_idx_to_symbol(row_idx);
            }
        }
    }
    consensus_string.into_iter().collect()
}

pub(crate) fn display_profile_matrix(profile_matrix: &[Vec<u32>]) {
    for (row_idx, row) in profile_matrix.iter().enumerate() {
        let symbol = row_idx_to_symbol(row_idx);
        println!(
            "{}: {}",
            symbol,
            row.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

fn row_idx_to_symbol(row_idx: usize) -> char {
    match row_idx {
        0 => Ok('A'),
        1 => Ok('C'),
        2 => Ok('G'),
        3 => Ok('T'),
        _ => Err("Invalid row index"),
    }
    .unwrap()
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
