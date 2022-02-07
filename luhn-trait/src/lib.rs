pub trait Luhn: ToString {
    fn valid_luhn(&self) -> bool {
        let luhn = self.to_string();
        let code = luhn.trim();
        if code.len() <= 1 || !code.chars().all(|ch| ch.is_digit(10) || ch.is_whitespace()) {
            return false;
        }

        let sum: u32 = code
            .chars()
            .rev()
            .filter_map(|ch| ch.to_digit(10))
            .enumerate()
            .map(|(i, n)| match i % 2 {
                0 => n,
                _ if n * 2 > 9 => n * 2 - 9,
                _ => n * 2,
            })
            .sum();
        sum % 2 == 0
    }
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {}
// where
//     T: ToString,
// {
//     fn valid_luhn(&self) -> bool {
//         let luhn = self.to_string();
//         let code = luhn.trim();
//         if code.len() <= 1 || !code.chars().all(|ch| ch.is_digit(10) || ch.is_whitespace()) {
//             return false;
//         }

//         let sum: u32 = code
//             .chars()
//             .rev()
//             .filter_map(|ch| ch.to_digit(10))
//             .enumerate()
//             .map(|(i, n)| match i % 2 {
//                 0 => n,
//                 _ if n * 2 > 9 => n * 2 - 9,
//                 _ => n * 2,
//             }sum();
//         sum % 2 == 0
//     }
// }
