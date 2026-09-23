fn main() {
	let v = vec![2,3,5,7,11,13,17];
	
	for (i, val) in v.iter().enumerate() {
		println!("{} {}", i, val);
	}
}