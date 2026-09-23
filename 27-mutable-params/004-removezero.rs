use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();

	set.insert(0);
    set.insert(1);
    set.insert(2);

    let result = remove_zero(set);
    println!("{:?}", result);
	
}

pub fn remove_zero(mut s: HashSet<i32>) -> HashSet<i32> {
	s.remove(&0);
    s
}