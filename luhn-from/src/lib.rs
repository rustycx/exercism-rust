pub struct Luhn {
    nums: Vec<u8>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        true
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        unimplemented!("From the given input '{}' create a new Luhn struct.", input);
    }
}


#[test]
fn testa() {
    println!("x={:?}", std::char::from_u32(0b100111000101101));
    let k = 'a'..'c';
    let full = "bookkeeping";
    let m = full[5..].contains("abc");
    let m = "找";
    let kk = m.split_at(1);
    println!("{}, {}", kk.0, kk.1);

}