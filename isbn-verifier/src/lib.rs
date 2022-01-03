/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn0(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let last_digit = isbn.chars().last().unwrap();
    if !last_digit.is_numeric() && last_digit != 'X' {
        return false;
    }

    let digits: Vec<u32> = isbn[..isbn.len() - 1]
        .chars()
        .filter(|ch| ch.is_numeric())
        .map(|ch| (ch as u32) - ('0' as u32))
        .collect();
    if digits.len() != 9 {
        return false;
    }
    let (sum, _) = digits.iter().fold((0, 10), |(mut sum, mut factor), n| {
        sum += factor * n;
        factor -= 1;
        (sum, factor)
    });
    match last_digit {
        'X' => (sum + 10) % 11 == 0,
        n => (sum + n as u32 - '0' as u32) % 11 == 0,
    }
}

// https://exercism.org/tracks/rust/exercises/isbn-verifier/solutions/DrJosh9000
pub fn is_valid_isbn1(isbn: &str) -> bool {
    let (count, sum, valid) = isbn.chars().fold((0, 0, true), |(count, sum, valid), c| match c {
        '0'..='9' => (count + 1, (sum + (10 - count) * c.to_digit(10).unwrap()), valid),
        'X' => (count + 1, sum + 10, valid && (count == 9)),
        '-' => (count, sum, valid),
        _ => (0, 0, false),
    });
    valid && count == 10 && sum % 11 == 0
}

// https://exercism.org/tracks/rust/exercises/isbn-verifier/solutions/elshize
pub fn is_valid_isbn2(isbn: &str) -> bool {
    match isbn
        .chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|(pos, c)| match (pos, c) {
            (p, '0'..='9') => Some((c.to_digit(10).unwrap() as usize * (10 - p), p)),
            (9, 'X') => Some((10, 9)),
            _ => None,
        })
        .try_fold((0, 0), |(sum, _), elem| elem.map(|(n, pos)| (sum + n, pos)))
    {
        Some((n, 9)) => n % 11 == 0,
        _ => false,
    }
}

pub fn is_valid_isbn3(isbn: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for ch in isbn.chars().filter(|&ch| ch != '-') {
        match ch {
            '0'..='9' => {
                sum += (10 - count) * ch.to_digit(10).unwrap() as usize;
                count += 1;
            }
            'X' if count == 9 => {
                sum += 10;
                count += 1;
            }
            _ => return false,
        }
    }
    count == 10 && sum % 11 == 0
}

// https://exercism.org/tracks/rust/exercises/isbn-verifier/solutions/ayzenquwe
pub fn is_valid_isbn(isbn: &str) -> bool {
    let re = regex::Regex::new(r"^[0-9]-?[0-9]{3}-?[0-9]{5}-?[0-9X]$").unwrap();
    re.is_match(isbn) && isbn.chars()
        .filter(|&c| c != '-')
        .zip((1..=10).rev())
        .fold(0, |acc, (n, i)| acc + i * n.to_digit(10).unwrap_or(10) as usize) % 11 == 0
}