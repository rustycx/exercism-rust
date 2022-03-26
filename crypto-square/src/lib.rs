pub fn encrypt0(input: &str) -> String {
    let k = input
        .chars()
        .flat_map(|ch| ch.to_lowercase())
        .filter(|ch| ch.is_alphanumeric())
        .collect::<Vec<char>>();
    let count = k.len();
    let r = (count as f64).sqrt() as usize;
    let c = if r * r < count { r + 1 } else { r };
    // k.extend(std::iter::repeat(' ').take(c * r - count));
    let mut ans = String::with_capacity(r * c);
    // k.chunks(18).for_each(|x| println!("{}", x.iter().collect::<String>()));
    for i in 0..c {
        for j in (i..c * r).step_by(c) {
            ans.push(*k.get(j).unwrap_or(&' '));
        }
        if i < c - 1 {
            ans.push(' ');
        }
    }
    ans
}

pub fn encrypt1(input: &str) -> String {
    let mut normalized = input
        .chars()
        .flat_map(|ch| ch.to_lowercase())
        .filter(|f| f.is_alphanumeric())
        .collect::<Vec<_>>();
    let k = (normalized.len() as f64).sqrt();
    let (c, r) = (k.ceil() as usize, k.floor() as usize);
    normalized.extend(&vec![' '; r * c - normalized.len()]);
    normalized
        .iter()
        .enumerate()
        .fold(vec![String::new(); c], |mut strs, (i, &ch)| {
            strs[i % c].push(ch);
            strs
        })
        .join(" ")
        .to_lowercase()
}

pub fn encrypt(input: &str) -> String {
    let pain = input
        .chars()
        .filter_map(|c| Some(c.to_ascii_lowercase()).filter(char::is_ascii_alphanumeric))
        .collect::<Vec<_>>();
    let width = (pain.len() as f64).sqrt().ceil() as usize;
    (0..width)
        .map(|i| {
            pain.chunks(width)
                .filter_map(|v| v.get(i).or(Some(&' ')))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
