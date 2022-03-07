use std::collections::HashMap;

pub fn can_construct_note0(magazine: &[&str], note: &[&str]) -> bool {
    fn word_count<'a>(words: &'a [&str]) -> HashMap<&'a str, usize> {
        let mut hash = HashMap::new();
        words.iter().for_each(|&m| {
            let entry = hash.entry(m).or_insert(0);
            *entry += 1;
        });
        hash
    }
    let mut m = word_count(magazine);
    for word in note {
        match m.entry(word).or_insert(0) {
            0 => return false,
            e @ _ => *e -= 1,
        }
    }
    true
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    fn word_count<'a>(words: &'a [&str]) -> HashMap<&'a str, usize> {
        let mut hash = HashMap::new();
        words.iter().for_each(|&m| {
            let entry = hash.entry(m).or_insert(0);
            *entry += 1;
        });
        hash
    }
    let magazine_words = word_count(magazine);
    let note_words = word_count(note);
    note_words.iter().all(|(w, c)|  magazine_words.get(w).unwrap_or(&0) >= c)
}

pub fn can_construct_note2(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = magazine.iter().fold(HashMap::new(), |mut words, &w| {
        *words.entry(w).or_insert(0) += 1;
        words
    });
    let note_words = note.iter().fold(HashMap::new(), |mut words, &w| {
        *words.entry(w).or_insert(0) += 1;
        words
    });
    note_words
        .iter()
        .all(|(w, count)| magazine_words.get(w).unwrap_or(&0) >= count)
}
