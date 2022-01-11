use std::collections::HashMap;

const STOP_CODON: &str = "stop codon";

pub struct CodonsInfo<'a> {
    codon_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_map.get(codon).copied()
    }

    pub fn of_rna0(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|seq| self.name_for(seq.unwrap()))
            .take_while(|&codon| codon != Some(STOP_CODON))
            .collect()
    }

    // https://exercism.org/tracks/rust/exercises/protein-translation/solutions/seikichi
    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut polypeptide = vec![];
        for i in (0..rna.len()).step_by(3) {
            if i + 3 > rna.len()  {
                return None;
            }
            match self.name_for(&rna[i..i + 3]) {
                None => return None,
                Some(STOP_CODON) => return Some(polypeptide),
                Some(name) => polypeptide.push(name),
            }
        }
        Some(polypeptide)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codon_map: pairs.iter().copied().collect(),
    }
}
