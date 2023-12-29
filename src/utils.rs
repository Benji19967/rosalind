use std::collections::HashMap;

/// <ID, DNA string>
pub type DnaStrings = HashMap<String, String>;

/// This should take in a file object, or a string, and return the
/// DNA strings in the file with their corresponding ID.
///
pub fn read_fasta(s: String) -> DnaStrings {
    let mut dna_strings: DnaStrings = HashMap::new();
    let mut first = true;
    let mut string_id: &str = "";
    let mut string_list: Vec<&str> = Vec::new();
    for line in s.lines() {
        if line.starts_with(">") {
            if first {
                string_id = get_string_id(line);
                first = false;
            } else {
                dna_strings.insert(string_id.into(), string_list.join(""));
                string_id = get_string_id(line);
                string_list = Vec::new();
            }
        } else {
            string_list.push(line);
        }
    }
    dna_strings.insert(string_id.into(), string_list.join(""));
    dna_strings
}

pub fn get_string_id(s: &str) -> &str {
    &s[1..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_fasta_simple() {
        let input_string: String = "\
            >Test_123\n\
            ACGT\n\
            GTGT\n\
            >Test_456\n\
            GGCC\n\
            CCGG"
            .to_string();
        assert_eq!(
            read_fasta(input_string),
            HashMap::from([
                ("Test_123".to_string(), "ACGTGTGT".to_string()),
                ("Test_456".to_string(), "GGCCCCGG".to_string())
            ])
        );
    }
}
