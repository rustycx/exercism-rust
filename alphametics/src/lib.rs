use std::collections::HashMap;

// https://exercism.org/tracks/rust/exercises/alphametics/solutions/denenr


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let v: Vec<&str> = input.split(" == ").collect();
    let sum = v[1].trim();
    let nums: Vec<&str> = v[0].split('+').map(|w| w.trim()).collect();
    None
}
