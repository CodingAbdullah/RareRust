use std::collections::HashSet;

fn main() {
	let v = vec![1,2,3];

    // ownership is lost here
	let s: HashSet<i32> = v.iter().copied().collect();

	println!("{:?}", v);
	println!("{:?}", s);
}