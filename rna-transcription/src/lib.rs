use core::panic;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

fn transform(ch: char) -> char {
    match ch {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => panic!("Invalid char in dna sequence"),
    }
}

impl Dna {
    pub fn new0(dna: &str) -> Result<Dna, usize> {
        match dna
            .chars()
            .position(|ch| ch != 'A' && ch != 'C' && ch != 'G' && ch != 'T')
        {
            Some(x) => Err(x),
            None => Ok(Dna(dna.to_string())),
        }
    }

    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.find(|c: char| ! "ACGT".contains(c)) {
            Some(pos) => Err(pos),
            None => Ok(Dna(dna.into()))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(self.0.chars().map(transform).collect::<String>().as_ref())
            .ok()
            .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .chars()
            .position(|ch| ch != 'A' && ch != 'C' && ch != 'G' && ch != 'U')
        {
            Some(x) => Err(x),
            None => Ok(Rna(rna.to_string())),
        }
    }
}
