/// "Encipher" with the Atbash cipher.
pub fn encode0(plain: &str) -> String {
    plain
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .map(|ch| {
            if ch.is_ascii_digit() {
                ch
            } else {
                (b'a' + b'z' - ch as u8) as char
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|w| String::from_iter(w))
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode0(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| {
            if ch.is_alphabetic() {
                (b'a' + b'z' - ch as u8) as char
            } else {
                ch
            }
        })
        .collect()
}

pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(swap)
        .collect::<Vec<_>>()
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .into_iter()
        .collect()
}
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(swap).collect()
}
fn swap(c: char) -> Option<char> {
    match c {
        '0'..='9' => Some(c),
        'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
        'A'..='Z' => Some((b'a' + b'Z' - c as u8) as char),
        _ => None,
    }
}
