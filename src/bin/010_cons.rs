use rosalind::utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = utils::read_input_from_stdin()?;
    let sequences = utils::read_fasta(input);
    let profile_matrix = build_profile_matrix(&sequences.values().collect()).unwrap();
    let consensus_string = build_consensus_string(&profile_matrix);
    println!("{:?}", consensus_string);
    display_profile_matrix(&profile_matrix);

    Ok(())
}

/// DNA Strings:
///
///     A T C
///     A G G
///
/// Profile Matrix:
///
/// A:  2 0 0
/// C:  0 0 1
/// G:  0 1 1
/// T:  0 1 0
///
fn build_profile_matrix(sequences: &Vec<&String>) -> Result<Vec<Vec<u32>>> {
    let len_sequences = sequences[0].len();
    let mut profile_matrix = vec![vec![0; len_sequences]; 4];
    for sequence in sequences {
        for (i, c) in sequence.chars().enumerate() {
            let index = "ACGT".find(c).unwrap();
            profile_matrix[index][i] += 1;
        }
    }
    Ok(profile_matrix)
}

fn build_consensus_string(profile_matrix: &[Vec<u32>]) -> String {
    let len_sequences = profile_matrix[0].len();
    let mut max_occurence: Vec<u32> = vec![0; len_sequences];
    let mut consensus_string: Vec<char> = vec!['A'; len_sequences];

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

fn display_profile_matrix(profile_matrix: &[Vec<u32>]) {
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

    #[test]
    fn cons1() {
        assert_eq!(build_profile_matrix(), vec![2, 4]);
    }
}
