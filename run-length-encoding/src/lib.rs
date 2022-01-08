pub fn encode(source: &str) -> String {
    let mut ans = String::new();
    let mut chars = source.chars().peekable();
    let mut cnt = 0;
    while let Some(ch) = chars.next() {
        cnt += 1;
        if chars.peek() != Some(&ch) {
            if cnt > 1 {
                ans.push_str(&cnt.to_string())
            }
            ans.push(ch);
            cnt = 0;
        }
    }
    ans
}

pub fn decode0(source: &str) -> String {
    let mut num = 0;
    let mut ans = String::new();
    for ch in source.chars() {
        if ch.is_numeric() {
            num = num * 10 + ch.to_digit(10).unwrap();
        } else {
            match num {
                0 => ans.push(ch),
                _ => ans.extend(std::iter::repeat(ch).take(num as usize)),
            }
            num = 0;
        }
    }
    ans
}

pub fn decode(source: &str) -> String {
    source
        .split(|c: char| !c.is_digit(10))
        .map(|cnt| cnt.parse::<usize>().unwrap_or(1))
        .zip(source.matches(|c: char| !c.is_digit(10)))
        .map(|(num, c)| c.repeat(num))
        .collect()
}
