use std::collections::HashSet;

fn main() {
	let mut s = HashSet::new();
	s.insert(1);
	s.insert(2);
	s.insert(3);
	
	let v: HashSet<i32> = s.into_iter().collect(); // Complete this
    println!("{:?}", v);
} 