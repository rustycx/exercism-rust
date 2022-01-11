use std::vec;

use itertools::iproduct;

pub fn find_saddle_points0(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut min_col = vec![u64::MAX; input[0].len()];
    for j in 0..input[0].len() {
        for i in 0..input.len() {
            min_col[j] = min_col[j].min(input[i][j]);
        }
    }
    let mut ans = vec![];
    for i in 0..input.len() {
        let row_max = match input[i].iter().max().copied() {
            Some(v) => v,
            None => return ans,
        };
        for j in 0..input[i].len() {
            if input[i][j] == row_max && min_col[j] == input[i][j] {
                ans.push((i, j));
            }
        }
    }
    ans
}

pub fn find_saddle_points1(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();
    let h = input.len();
    let w = input[0].len();
    for i in 0..h {
        for j in 0..w {
            let row = &input[i];
            let max = row.iter().max().unwrap();
            let min = input.iter().map(|x| x[j]).min().unwrap();
            let value = input[i][j];
            if value >= *max && value <= min {
                ans.push((i, j));
            }
        }
    }
    ans
}

// https://exercism.org/tracks/rust/exercises/saddle-points/solutions/akiekintveld
pub fn find_saddle_points2(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &v)| (r, c, v)))
        .filter_map(|(r, c, v)| {
            if input.iter().all(|row| v <= row[c]) && input[r].iter().all(|&x| v >= x) {
                Some((r, c))
            } else {
                None
            }
        })
        .collect()
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = input.len();
    if height < 1 {
        return Vec::new();
    }
    let width = input[0].len();
    let maxs: Vec<u64> = input
        .iter()
        .map(|r| r.iter().max().unwrap_or(&u64::min_value()))
        .copied()
        .collect();
    // get the minimum value for each row
    let mins: Vec<u64> = (0..width)
        .map(|c| *(input.iter().map(|r| &r[c]).min().unwrap_or(&u64::max_value())))
        .collect();
    iproduct!(0..height, 0..width)
        .filter(|&(x, y)| input[x][y] == mins[y] && input[x][y] == maxs[x])
        .collect()
}
