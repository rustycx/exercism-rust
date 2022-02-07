pub fn number0(user_number: &str) -> Option<String> {
    let number = user_number
        .chars()
        .filter(|ch| ch.is_digit(10))
        .skip_while(|&c| c == '1')
        .collect::<Vec<char>>();
    if number.len() == 10 && (number[0] != '0' && number[0] != '1') && (number[3] != '0' && number[3] != '1') {
        Some(number.iter().collect())
    } else {
        None
    }
}

pub fn number(user_number: &str) -> Option<String> {
    let number = user_number
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .skip_while(|&c| c == 1)
        .collect::<Vec<_>>();
    if number.len() == 10 && number[0] > 1 && number[3] > 1 {
        number.iter().map(|&n| char::from_digit(n, 10)).collect()
    } else {
        None
    }
}

// https://exercism.org/tracks/rust/exercises/phone-number/solutions/rsalmei
pub fn number1(user_number: &str) -> Option<String> {
    let iter = user_number
        .chars()
        .filter(|ch| ch.is_digit(10))
        .skip_while(|&c| c == '1');
    (iter.clone().count() == 10 && iter.clone().nth(0) >= Some('2') && iter.clone().nth(3) >= Some('2'))
        .then(|| iter.collect())
}

// https://exercism.org/tracks/rust/exercises/phone-number/solutions/almondtools
pub fn number2(user_number: &str) -> Option<String> {
    let phone_number = user_number.chars().filter(|c| c.is_digit(10)).collect::<String>();
    match phone_number.len() {
        11 if valid11(&phone_number) => Some(phone_number[1..].to_string()),
        10 if valid10(&phone_number) => Some(phone_number),
        7 if phone_number.starts_with(number2to9) => Some(phone_number),
        _ => None,
    }
}

fn valid11(s: &str) -> bool {
    s.starts_with('1') && s.chars().nth(1).map_or(false, number2to9) && s.chars().nth(4).map_or(false, number2to9)
}

fn valid10(s: &str) -> bool {
    s.starts_with(number2to9) && s.chars().nth(3).map_or(false, number2to9)
}

fn number2to9(c: char) -> bool {
    ('2'..='9').contains(&c)
}
