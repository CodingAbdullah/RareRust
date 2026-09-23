use std::collections::HashSet;

fn main() {
	let hs = HashSet::from([1,2,3]);

	for e in hs.iter() {
		println!("{}", *e);
	}

	let sum: i32 = hs.iter().sum();

	println!("{}", sum);
}