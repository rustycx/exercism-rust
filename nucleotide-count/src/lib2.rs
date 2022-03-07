use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    }
    let mut cnt = 0;
    for ch in dna.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => {
                if ch == nucleotide {
                    cnt += 1;
                }
            },
            _ => return Err(ch),
        }
    }
    Ok(cnt)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut ans = HashMap::new();
    ans.insert('A', 0);
    ans.insert('C', 0);
    ans.insert('G', 0);
    ans.insert('T', 0);
    for ch in dna.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => *ans.entry(ch).or_default() += 1,
            _ => return Err(ch),
        }
    }
    Ok(ans)
}
