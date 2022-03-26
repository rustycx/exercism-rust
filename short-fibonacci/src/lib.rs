/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci0() -> Vec<u8> {
    let mut fib = Vec::with_capacity(5);
    let mut a = 0;
    let mut b = 1;
    for _ in 0..5 {
        (a, b) = (b, a + b);
        fib.push(a);
    }
    fib
}

pub fn fibonacci() -> Vec<u8> {
    let mut fib = create_buffer(5);
    for i in 0..5 {
        match i {
            0 | 1 => fib[i] = 1,
            _ => fib[i] = fib[i-1] + fib[i-2],
        }
    }
    fib
}
