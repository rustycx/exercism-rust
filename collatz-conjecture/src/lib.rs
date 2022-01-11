pub fn collatz0(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    loop {
        if n == 1 {
            return Some(steps);
        }
        if n % 2 == 0 {
            n /= 2;
        } else if (u64::MAX - 1) / 3 < n {
            return None;
        } else {
            n = n * 3 + 1;
        }
        steps += 1;
    }
}

pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    loop {
        if n == 1 {
            return Some(steps);
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        steps += 1;
    }
}
