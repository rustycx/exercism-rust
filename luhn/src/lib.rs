/// Check a Luhn checksum.
pub fn is_valid1(mut code: &str) -> bool {
    code = code.trim();
    if code.len() <= 1 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in code.chars().filter(|&x| x != ' ').rev().enumerate() {
        if !c.is_digit(10) {
            return false;
        }
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            sum += digit;
        } else {
            sum += if digit > 4 { digit * 2 - 9 } else { digit * 2 };
        }
    }
    sum % 10 == 0
}

pub fn is_valid2(code: &str) -> bool {
    let code = code.trim();
    if code.len() < 2 || !code.chars().all(|ch| ch.is_digit(10) || ch.is_whitespace()) {
        return false;
    }

    let sum: u32 = code
        .chars()
        .rev()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .map(|(i, n)| match i % 2 {
            0 => n,
            _ if n * 2 > 9 => n * 2 - 9,
            _ => n * 2,
        })
        .sum();
    sum % 2 == 0
}

// https://exercism.org/tracks/rust/exercises/luhn/solutions/JaneL
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 0 { num } else { num * 2 })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}