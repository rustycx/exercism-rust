use Direction::*;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn can_move_to(a: i32, b: i32, d: Direction, size: i32, grid: &mut [&mut [u32]]) -> bool {
    let (tx, ty) = match d {
        Right => (a, b + 1),
        Down => (a + 1, b),
        Left => (a, b - 1),
        Up => (a - 1, b),
    };
    tx >= 0 && tx < size && ty >= 0 && ty < size && grid[tx as usize][ty as usize] == 0
}

pub fn spiral_matrix0(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    let mut matrix = vec![0; size as usize * size as usize];
    let mut matrix: Vec<_> = matrix.as_mut_slice().chunks_mut(size as usize).collect();
    let grid = matrix.as_mut_slice();
    let mut d = Right;
    let mut a = 0;
    let mut b = 0;
    grid[0][0] = 1;
    let mut n = 2;
    while n <= size * size {
        if can_move_to(a, b, d, size as i32, grid) {
            match d {
                Right => b += 1,
                Down => a += 1,
                Left => b -= 1,
                Up => a -= 1,
            }
            grid[a as usize][b as usize] = n;
            n += 1;
        } else {
            d = match d {
                Right => Down,
                Down => Left,
                Left => Up,
                Up => Right,
            }
        }
    }

    matrix.iter().map(|x| x.iter().copied().collect::<Vec<u32>>()).collect()
}

// https://exercism.org/tracks/rust/exercises/spiral-matrix/solutions/denenr
const VECTORS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub fn spiral_matrix1(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut movement = VECTORS.iter().cycle();
    let (mut x, mut y, mut n) = (0, -1, 1..);
    for (move_x, move_y) in std::iter::once(size)
        .chain((1..size).rev().flat_map(|n| std::iter::repeat(n).take(2)))
        .flat_map(|steps| std::iter::repeat(movement.next().unwrap()).take(steps))
    {
        x += move_x;
        y += move_y;
        matrix[x as usize][y as usize] = n.next().unwrap();
    }
    matrix
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let (mut left, mut right, mut top, mut bottom) = (0, size as i32 - 1, 0, size as i32 - 1);
    let mut n = 1u32;
    while top <= bottom && left <= right {
        for j in left..=right {
            matrix[top as usize][j as usize] = n;
            n += 1;
        }
        top += 1;
        for i in top..=bottom {
            matrix[i as usize][right as usize] = n;
            n += 1;
        }
        right -= 1;
        for j in (left..=right).rev() {
            matrix[bottom as usize][j as usize] = n;
            n += 1;
        }
        bottom -= 1;
        for i in (top..=bottom).rev() {
            matrix[i as usize][left as usize] = n;
            n += 1;
        }
        left += 1;
    }
    matrix
}
