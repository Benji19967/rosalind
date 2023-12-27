use phf::phf_map;
use std::io::{self, Read};

use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static RNA_CODON_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    "UUU" => "F",
    "CUU" => "L",
    "AUU" => "I",
    "GUU" => "V",
    "UUC" => "F",
    "CUC" => "L",
    "AUC" => "I",
    "GUC" => "V",
    "UUA" => "L",
    "CUA" => "L",
    "AUA" => "I",
    "GUA" => "V",
    "UUG" => "L",
    "CUG" => "L",
    "AUG" => "M",
    "GUG" => "V",
    "UCU" => "S",
    "ACU" => "T",
    "CCU" => "P",
    "GCU" => "A",
    "UCC" => "S",
    "CCC" => "P",
    "ACC" => "T",
    "GCC" => "A",
    "UCA" => "S",
    "CCA" => "P",
    "ACA" => "T",
    "GCA" => "A",
    "UCG" => "S",
    "CCG" => "P",
    "ACG" => "T",
    "GCG" => "A",
    "UAU" => "Y",
    "CAU" => "H",
    "AAU" => "N",
    "GAU" => "D",
    "UAC" => "Y",
    "CAC" => "H",
    "GAC" => "D",
    "AAC" => "N",
    "UAA" => "Stop",
    "CAA" => "Q",
    "AAA" => "K",
    "GAA" => "E",
    "UAG" => "Stop",
    "CAG" => "Q",
    "AAG" => "K",
    "GAG" => "E",
    "UGU" => "C",
    "CGU" => "R",
    "AGU" => "S",
    "GGU" => "G",
    "UGC" => "C",
    "CGC" => "R",
    "AGC" => "S",
    "GGC" => "G",
    "UGA" => "Stop",
    "CGA" => "R",
    "AGA" => "R",
    "GGA" => "G",
    "UGG" => "W",
    "CGG" => "R",
    "AGG" => "R",
    "GGG" => "G",
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    for line in input.lines() {
        let protein_string = encode(line);
        println!("{}", protein_string);
    }

    Ok(())
}

fn encode(s: &str) -> String {
    let mut protein_string_chars: Vec<&str> = vec![];
    for chunk in &s.chars().chunks(3) {
        let codon: String = chunk.collect();
        protein_string_chars.push(RNA_CODON_TABLE[&codon])
    }
    let protein_string: String = protein_string_chars.join("");
    protein_string
}

#[cfg(test)]
mod tests {
    use crate::encode;

    #[test]
    fn case1() {
        assert_eq!(encode("AUG"), "M");
    }
}
