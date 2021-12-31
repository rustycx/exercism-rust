use std::collections::HashMap;
use rayon::prelude::*;

fn process_slice(input: &[&str]) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|w| w.chars())
        .filter(|ch| ch.is_alphabetic())
        .fold(HashMap::new(), |mut h, ch| {
            *h.entry(ch.to_ascii_lowercase()).or_default() += 1;
            h
        })
}
/*
test bench_large_parallel   ... bench:     209,647 ns/iter (+/- 68,390)
test bench_large_sequential ... bench:     605,003 ns/iter (+/- 57,732)
test bench_small_parallel   ... bench:      15,795 ns/iter (+/- 2,501)
test bench_small_sequential ... bench:      21,121 ns/iter (+/- 6,757)
test bench_tiny_parallel    ... bench:         115 ns/iter (+/- 6)
test bench_tiny_sequential  ... bench:          98 ns/iter (+/- 7)
*/
pub fn frequency<'a>(input: &'a [&str], worker_count: usize) -> HashMap<char, usize> {
    let mut ans = HashMap::new();
    if input.len() < 200 {
        return process_slice(input);
    }
    let chunk_size = input.len() / worker_count;
    let mut handles = vec![];
    for i in (0..input.len()).step_by(chunk_size) {
        let slice = &input[i..(i + chunk_size).min(input.len())];
        let slice = unsafe { std::mem::transmute::<&'a [&str], &'static [&str]>(slice) };
        let handle = std::thread::spawn(|| process_slice(slice));
        handles.push(handle);
    }
    for h in handles {
        for (k, v) in h.join().unwrap() {
            *ans.entry(k).or_default() += v;
        }
    }
    ans
}

/*
test bench_large_parallel   ... bench:     516,468 ns/iter (+/- 12,546)
test bench_large_sequential ... bench:     601,447 ns/iter (+/- 72,552)
test bench_small_parallel   ... bench:      84,414 ns/iter (+/- 3,981)
test bench_small_sequential ... bench:      20,424 ns/iter (+/- 1,216)
test bench_tiny_parallel    ... bench:      23,132 ns/iter (+/- 1,647)
test bench_tiny_sequential  ... bench:         121 ns/iter (+/- 32)
*/
pub fn frequency1(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input: Vec<char> = input
        .iter()
        .flat_map(|w| w.chars())
        .filter(|ch| ch.is_alphabetic())
        .flat_map(|ch| ch.to_lowercase())
        .collect();
    (0..worker_count)
        .into_par_iter()
        .map(|i| {
            input
                .iter()
                .skip(i)
                .step_by(worker_count)
                .fold(HashMap::new(), |mut h, &ch| {
                    *h.entry(ch).or_default() += 1;
                    h
                })
        })
        .reduce(HashMap::new, |mut ans, m| {
            m.iter().for_each(|(&k, &v)| {
                *ans.entry(k).or_insert(0) += v;
            });
            ans
        })
}
