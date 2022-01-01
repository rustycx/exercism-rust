#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify0(num: u64) -> Option<Classification> {
    use crate::Classification::*;
    let sum = (1..).take_while(|n| n * n <= num).fold(0, |mut sum, n| {
        if num % n == 0 {
            sum += n;
            if n != 1 && n * n != num {
                sum += num / n;
            }
        }
        sum
    });
    match sum {
        _x @ 0 => None,
        _x @ 1 => Some(Deficient),
        x if x > 1 && x < num => Some(Deficient),
        x if x == num => Some(Perfect),
        _ => Some(Abundant),
    }
}

// https://exercism.org/tracks/rust/exercises/perfect-numbers/solutions/lonesometraveler
pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2).filter(|i| num % i == 0).sum::<u64>() {
        _ if num == 0 => None,
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}
