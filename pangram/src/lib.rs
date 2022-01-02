use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    26 == sentence
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .fold(HashSet::new(), |mut set, ch| {
            set.insert(ch.to_ascii_lowercase());
            set
        })
        .len()
}
