use std::collections::HashSet;

pub fn check0(candidate: &str) -> bool {
    candidate
        .matches(|c| c != '-' && c != ' ')
        .fold(HashSet::new(), |mut h, c| {
            h.insert(c.to_lowercase());
            h
        })
        .len()
        == candidate.chars().filter(|&c| c != '-' && c != ' ').count()
}

// https://exercism.org/tracks/rust/exercises/isogram/solutions/hansrodtang
pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::<char>::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}