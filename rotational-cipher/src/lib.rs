pub fn rotate(input: &str, key: i8) -> String {
    let transform = |ch: i8, base: i8| -> char {
        let k = (ch - base + key).rem_euclid(26) + base;
        k as u8 as char
    };

    input
        .chars()
        .map(|ch| match ch {
            'a'..='z' => transform(ch as i8, b'a' as i8),
            'A'..='Z' => transform(ch as i8, b'A' as i8),
            _ => ch,
        })
        .collect()
}
