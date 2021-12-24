pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // std::iter::empty()
    // iter.step_by(2)
    // iter.enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, v)| v)
    iter.enumerate().filter(|x| x.0 % 2 == 0).map(|x| x.1)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
