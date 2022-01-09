pub fn get_diamond0(c: char) -> Vec<String> {
    let n = c as usize - 'A' as usize;
    let size = n * 2 + 1;
    let mut ans = vec![vec![' '; size]; size];
    let mut chars = ('A'..=c).chain(('A'..c).rev());
    for (i, j) in (0..=n).chain((0..n).rev()).enumerate() {
        let ch = chars.next().unwrap();
        ans[i][n - j] = ch;
        ans[i][n + j] = ch;
    }
    ans.iter().map(|row| row.iter().collect::<String>()).collect()
}

// https://exercism.org/tracks/rust/exercises/diamond/solutions/mbillingr
pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as i8;
    (-n..=n)
        .map(|row| {
            (-n..=n)
                .map(|col| {
                    if row.abs() + col.abs() == n {
                        (col.abs() as u8 + b'A') as char
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect()
}
