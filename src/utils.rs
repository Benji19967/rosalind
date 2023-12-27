// TODO: What should the return format be?
//
// 1. Vec<(id, s)
// 2. Vec<(DnaString{id: <id>, s: <String>})
// 3. HashMap<id, s>

/// This should take in a file object, or a string, and return the
/// DNA strings in the file with their corresponding ID.
///
pub fn read_fasta() {
    unimplemented!()
}

// TODO: How to create a string with several lines, for testing?

#[cfg(test)]
mod tests {
    use crate::utils::read_fasta;

    #[test]
    fn read_fasta_simple() {
        assert_eq!(read_fasta(), "");
    }
}
