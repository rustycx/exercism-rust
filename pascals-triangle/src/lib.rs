use std::iter;

pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows0(&self) -> Vec<Vec<u32>> {
        let mut ans = vec![];
        for i in 0..self.0 as usize {
            if i == 0 {
                ans.push(vec![1]);
            } else {
                let mut row = vec![1];
                for j in 1..i {
                    row.push(ans[i - 1][j - 1] + ans[i - 1][j]);
                }
                row.push(1);
                ans.push(row);
            }
        }
        ans
    }

    pub fn rows1(&self) -> Vec<Vec<u32>> {
        match self.0 {
            0 => Vec::new(),
            1 => vec![vec![1]],
            r => {
                let mut rows = Self::new(r - 1).rows();
                let new_row = {
                    let last_row = rows.last().unwrap();
                    let zip1 = iter::once(0).chain(last_row.iter().cloned());
                    let zip2 = last_row.iter().cloned().chain(iter::once(0));
                    zip1.zip(zip2).map(|(n, m)| n + m).collect()
                };
                rows.push(new_row);
                rows
            }
        }
    }

    pub fn rows2(&self) -> Vec<Vec<u32>> {
        #[allow(mutable_borrow_reservation_conflict)]
        fn calc(r: u32) -> Vec<u32> {
            let mut ret = vec![1];
            for p in 1..(r + 1) {
                let last = ret.last().unwrap();
                ret.push((last * (r + 1 - p)) / p)
            }
            ret
        }
        (0..self.0).map(|r| calc(r)).collect()
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        fn step(cur: &Vec<u32>) -> Vec<u32> {
            let mut result = vec![1];
            for i in cur.windows(2).map(|x| x.iter().sum()) {
                result.push(i);
            }
            result.push(1);
            result
        }
        if self.0 == 0 {
            return vec![];
        };
        let mut result = vec![vec![1]];
        for _ in 1..self.0 {
            let next = step(result.last().unwrap());
            result.push(next);
        }
        result
    }
}
