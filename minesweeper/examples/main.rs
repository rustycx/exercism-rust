fn main() {
    let k: Vec<i32> = (0..3).take(20).skip(0).collect();
    println!("{:?}", k);

    let slice = ['l', 'o', 'r', 'e', 'm'];
    let iter = slice.chunks(2);
	let k: Vec<String> = iter.map(String::from_iter).collect();
    println!("{:?}", k);
}
