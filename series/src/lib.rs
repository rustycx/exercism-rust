pub fn series0(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".into(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| String::from_iter(w))
        .collect()
}

pub fn series1(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
// https://exercism.org/tracks/rust/exercises/series/solutions/markcatley
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let result_length = digits.len() + 1 - len;
    (0..result_length)
        .map(|x| digits.chars().skip(x).take(len).collect())
        .collect()
}
