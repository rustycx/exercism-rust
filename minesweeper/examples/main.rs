fn main() {
    let k: Vec<i32> = (0..3).take(20).skip(1).collect();
    println!("abc: {:?}", k);

    let slice = ['l', 'o', 'r', 'e', 'm'];
    let iter = slice.chunks(2);
	// let k: Vec<String> = iter.map(String::from_iter).collect();
	let k: Vec<String> = iter.map(|x| x.iter().collect()).collect();
    println!("{:?}", k);
}
