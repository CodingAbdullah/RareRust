fn main() {
	// make v mutable so this compiles
	let mut v = vec![1,2,3];
	
	let result = max(&mut v);
	println!("{:?}", result);
}

// no actual mutation happens here
pub fn max(v: &mut Vec<i32>) -> Option<i32> {
    v.iter().max().copied()
}