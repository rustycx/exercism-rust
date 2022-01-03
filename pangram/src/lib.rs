use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram0(sentence: &str) -> bool {
    26 == sentence
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii() && ch.is_alphabetic())
        .collect::<HashSet<char>>()
        .len()
}

pub fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<char> = (b'a'..=b'z').map(|ch| ch as char).collect();
    let sentence_letters: HashSet<char> = sentence
        .chars()
        .filter(|ch| ch.is_ascii())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    sentence_letters.is_superset(&letters)
}
