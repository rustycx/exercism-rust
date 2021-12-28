use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    word_chars.sort_unstable();
    let lower_word = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter(|w| w.to_lowercase() != lower_word)
        .filter(|w| {
            let mut anagram_chars = w.to_lowercase().chars().collect::<Vec<char>>();
            anagram_chars.sort_unstable();
            word_chars == anagram_chars
        })
        .copied()
        .collect()
}

pub fn anagrams_for2<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let counts = word
        .to_lowercase()
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut h, c| {
            *h.entry(c).or_default() += 1;
            h
        });
    let word_lowercase = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter(|w| w.to_lowercase() != word_lowercase)
        .filter(|w| {
            let char_count = w
                .to_lowercase()
                .chars()
                .fold(HashMap::<char, usize>::new(), |mut h, c| {
                    *h.entry(c).or_default() += 1;
                    h
                });
            char_count == counts
        })
        .cloned()
        .collect()
}
