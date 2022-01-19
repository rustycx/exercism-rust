use std::collections::HashSet;

pub fn find0(sum: u32) -> HashSet<[u32; 3]> {
    let mut ans = HashSet::new();
    for a in 3..sum / 3 {
        for b in a + 1..=(sum - a) / 2 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                ans.insert([a, b, c]);
            }
        }
    }
    ans
}

pub fn find1(sum: u32) -> HashSet<[u32; 3]> {
    (1..=(sum - 3) / 3)
        .flat_map(|a| (a + 1..=(sum - a - 1) / 2).map(move |b| [a, b, sum - a - b]))
        .filter(|&[a, b, c]| a * a + b * b == c * c)
        .collect()
}

// https://exercism.org/tracks/rust/exercises/pythagorean-triplet/solutions/nerigardu
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    if sum % 2 != 0 {
        return HashSet::new();
    }
    (3..)
        .filter(|a| sum * a % (sum - a) == 0)
        // .take_while(|a| (sum - a).pow(2) > sum.pow(2) / 2)
        .take_while(|a| sum.pow(2) > 2 * a * (2 * sum - a))
        .map(|a| {
            let b = (sum / 2) * (sum - 2 * a) / (sum - a);
            let c = sum - (a + b);
            [a, b, c]
        })
        .collect()
}
