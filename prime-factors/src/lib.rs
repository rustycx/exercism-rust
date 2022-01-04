pub fn factors0(mut n: u64) -> Vec<u64> {
    let mut i = 2;
    let mut ans = vec![];
    while n > 1 {
        while n % i == 0 {
            ans.push(i);
            n /= i;
        }
        i += 1;
    }
    ans
}

pub fn factors(n: u64) -> Vec<u64> {
    fn recursive(n: u64, factor: u64) -> Vec<u64> {
        let mut ans = Vec::new();
        if n % factor == 0 {
            ans.push(factor);
            ans.append(&mut recursive(n / factor, factor));
        } else {
            match (factor + 1..=n).find(|&x| n % x == 0) {
                Some(x) => {
                    ans.push(x);
                    ans.append(&mut recursive(n/x, x));
                }
                None => {}
            }

        }
        ans
    }
    recursive(n, 2)
}
