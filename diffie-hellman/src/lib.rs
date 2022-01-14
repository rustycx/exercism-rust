use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn mul_mod(a: u64, b: u64, c: u64) -> u64 {
    (a as u128 * b as u128 % c as u128) as u64
}

fn pow_mod(mut x: u64, mut y: u64, p: u64) -> u64 {
    let mut ans = 1;
    while y != 0 {
        if y % 2 == 1 {
            ans = mul_mod(ans, x, p);
        }
        x = mul_mod(x, x, p);
        y >>= 1;
    }
    ans
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub, a, p)
}
