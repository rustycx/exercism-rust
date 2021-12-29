#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => { if contains(a, b) { Superlist } else { Unequal } }
        (m, n) if m < n => { if contains(b, a) { Sublist } else { Unequal } }
        (_, _) => { if a == b { Equal } else { Unequal } }
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|w| w == b)
}
