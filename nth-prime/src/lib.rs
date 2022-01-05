pub fn nth0(n: u32) -> u32 {
    (2..)
        .filter(|&n| {
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 1;
            }
            return true;
        })
        .nth(n as usize)
        .unwrap()
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(n as usize + 1);
    (2..)
        .filter(|n| {
            if primes.iter().take_while(|&p| n / p >= *p).all(|p| n % p != 0) {
                primes.push(*n);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
