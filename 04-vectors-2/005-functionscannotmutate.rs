fn main() {
	let mut v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    // v.push(6); // comment this out
	v
} 