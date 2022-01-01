pub fn is_armstrong_number0(num: u32) -> bool {
    let mut len = 0;
    let mut tmp = num;
    while tmp > 0 {
        len += 1;
        tmp /= 10;
    }
    let mut arm = 0;
    tmp = num;
    while tmp > 0 {
        let rem = tmp % 10;
        tmp /= 10;
        arm += rem.pow(len);
    }
    arm == num
}

pub fn is_armstrong_number1(num: u32) -> bool {
    fn digits(mut num: u32) -> Vec<u32> {
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits
    }
    let digits = digits(num);
    let len = digits.len();

    num == digits.iter().map(|d| d.pow(len as u32)).sum::<u32>()
}

// https://exercism.org/tracks/rust/exercises/armstrong-numbers/solutions/wezm
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = (num as f64).log10() as u32 + 1;
    num == (0..digits).map(|i| (num / 10u32.pow(i) % 10).pow(digits)).sum::<u32>()
}
