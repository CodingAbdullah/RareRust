use std::collections::HashSet;

fn main() {
	let v = Vec::from([2,4,8,10]);
	let s: HashSet<i32> = v.into_iter().collect();

    println!("{:?}", s); // Added for stdout testing
} 