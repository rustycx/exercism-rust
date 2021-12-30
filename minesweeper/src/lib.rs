use std::vec;

pub fn annotate1(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }
    let (m, n) = (minefield.len(), minefield[0].len());
    let mut ans = Vec::with_capacity(m);
    for i in 0..m {
        let mut row: Vec<char> = Vec::with_capacity(n);
        for j in 0..n {
            row.push(mines_at(minefield, i, j));
        }
        ans.push(row.into_iter().collect());
    }
    ans
}

#[rustfmt::skip]
const MASK: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
// https://exercism.org/tracks/rust/exercises/minesweeper/solutions/kstep
pub fn annotate2(minefield: &[&str]) -> Vec<String> {
    let h = minefield.len() as i32;
    (0..h)
        .map(|x| {
            let w = minefield[x as usize].len() as i32;
            (0..w)
                .map(|y| {
                    if minefield[x as usize].as_bytes()[y as usize] == b'*' {
                        '*'
                    } else {
                        match MASK
                            .iter()
                            .map(|&(dx, dy)| (dx + x, dy + y))
                            .filter(|&(x, y)| {
                                0 <= x
                                    && x < h
                                    && 0 <= y
                                    && y < w
                                    && minefield[x as usize].as_bytes()[y as usize] == b'*'
                            })
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}

// https://exercism.org/tracks/rust/exercises/minesweeper/solutions/McGittyHub
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    if minefield[0].is_empty() {
        return vec![String::new()];
    }
    let width = minefield[0].len();
    let mut result = Vec::new();
    for (y, &line) in minefield.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                result.push('*');
                continue;
            }
            let neighbor_mines = minefield
                .iter()
                .take(y + 2)
                .skip((y as isize - 1).max(0) as usize)
                .flat_map(|&line| line.chars().take(x + 2).skip((x as isize - 1).max(0) as usize))
                .filter(|&chr| chr == '*')
                .count();
            if neighbor_mines > 0 {
                result.push(format!("{}", neighbor_mines).chars().nth(0).unwrap());
            } else {
                result.push(' ');
            }
        }
    }
    result.chunks(width).map(String::from_iter).collect()
}

fn mines_at(minefield: &[&str], i: usize, j: usize) -> char {
    if minefield[i].chars().nth(j).unwrap() == '*' {
        return '*';
    }
    let (m, n) = (minefield.len(), minefield[0].len());
    let mut cnt = 0u8;
    for a in -1..=1 {
        for b in -1..=1 {
            let (r, c) = (i as i32 + a, j as i32 + b);
            if r >= 0
                && r < (m as i32)
                && c >= 0
                && c < (n as i32)
                && minefield[r as usize].chars().nth(c as usize).unwrap() == '*'
            {
                cnt += 1;
            }
        }
    }
    if cnt > 0 {
        (('0' as u8) + cnt) as char
    } else {
        ' '
    }
}
