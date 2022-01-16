#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp0(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    } else if let Some(k) = string_digits.find(|ch: char| !ch.is_digit(10)) {
        return Err(Error::InvalidDigit(string_digits.chars().nth(k).unwrap()));
    } else if span == 0 {
        return Ok(1);
    }
    let mut ans = u64::MIN;
    for w in string_digits.chars().collect::<Vec<char>>().windows(span) {
        ans = ans.max(w.iter().fold(1, |mut p, &v| {
            p *= v.to_digit(10).unwrap() as u64;
            p
        }));
    }
    Ok(ans)
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ => Ok(string_digits
            .chars()
            .map(|ch| ch.to_digit(10).ok_or(Error::InvalidDigit(ch)))
            .collect::<Result<Vec<u32>, Error>>()?
            .windows(span)
            .map(|w| w.iter().product::<u32>())
            .max()
            .ok_or(Error::SpanTooLong)? as u64),
    }
}
