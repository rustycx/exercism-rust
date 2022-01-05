use std::collections::HashMap;
use lazy_static::lazy_static;

fn get_score(c: char) -> u64 {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}

pub fn score0(word: &str) -> u64 {
    word.chars().map(|ch| get_score(ch.to_ascii_uppercase())).sum()
}

pub fn score1(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(get_score)
        .fold(0, std::ops::Add::add)
}

macro_rules! map {
    ( $({$($key:expr),+} => $val:expr),+ ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            $(map.insert($key, $val);)+
        )+
        map
    }};
}

lazy_static! {
    static ref LETTER_VALUES: HashMap<char, u64> = map! {
        { 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T' } => 1,
        { 'D', 'G'                                         } => 2,
        { 'B', 'C', 'M', 'P'                               } => 3,
        { 'F', 'H', 'V', 'W', 'Y'                          } => 4,
        { 'K'                                              } => 5,
        { 'J', 'X'                                         } => 8,
        { 'Q', 'Z'                                         } => 10
    };
}

pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase()
        .chars()
        .map(|c| LETTER_VALUES.get(&c).cloned().unwrap_or(0))
        .sum()
}
