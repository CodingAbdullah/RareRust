use std::collections::HashSet;

fn main() {
	let s = HashSet::from([2,4,8,10]);
	
	// YOUR CODE HERE
	let s_iter = s.into_iter();
	for item in s_iter {
		println!("{}", item);
	}
} 