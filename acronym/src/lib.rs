pub fn abbreviate0(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'])
        .fold(vec![], |mut v, w| {
            if !w.is_empty() {
                if w.chars()
                    .filter(|ch| ch.is_ascii_alphabetic())
                    .all(|ch| ch.is_ascii_uppercase())
                {
                    v.push(w.chars().next().unwrap())
                } else {
                    for (i, ch) in w.char_indices() {
                        if i == 0 || ch.is_ascii_uppercase() {
                            v.push(ch.to_ascii_uppercase());
                        }
                    }
                }
            }
            v
        })
        .iter()
        .collect()
}

pub fn abbreviate1(phrase: &str) -> String {
    let mut abbr = String::new();
    for word in phrase.split(&[' ', '-', '_', ':']) {
        if word.is_empty() {
            // Do nothing
        } else if word.chars().all(char::is_uppercase) {
            abbr.push(word.chars().next().unwrap());
        } else if word.chars().all(char::is_lowercase) {
            abbr.push(word.chars().next().unwrap().to_ascii_uppercase());
        } else {
            abbr.push_str(&word.chars().filter(|c| c.is_ascii_uppercase()).collect::<String>());
        }
    }
    abbr
}

// https://exercism.org/tracks/rust/exercises/acronym/solutions/menski
pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|ch: char| ch.is_whitespace() || ch == '-' || ch == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
