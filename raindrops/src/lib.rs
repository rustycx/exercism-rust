pub fn raindrops0(n: u32) -> String {
    let mut ans = String::new();
    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }
    if ans.is_empty() {
        ans = n.to_string()
    }
    ans
}

pub fn raindrops(num: i64) -> String {
    let mut ans: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter_map(|(n, s)| if num % n == 0 { Some(s) } else { None })
        .collect();
    if ans.len() == 0 {
        ans = num.to_string();
    }
    ans
}
