use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count0(words: &str) -> HashMap<String, u32> {
    words
        .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
        .map(|w| w.trim_matches(|ch: char| !ch.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut h, w| {
            *h.entry(w).or_insert(0) += 1;
            h
        })
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split_terminator(&[' ', '\t', '\n', ':', ',', '.'][..])
        // .split(&[' ', '\t', '\n', ':', ','][..])
        .filter_map(|w| match w.trim_matches(|ch: char| !ch.is_alphanumeric()) {
            w if w.is_empty() => None,
            w => Some(w),
        })
        .fold(HashMap::new(), |mut h, w| {
            *h.entry(w.to_string()).or_insert(0) += 1;
            h
        })
}
