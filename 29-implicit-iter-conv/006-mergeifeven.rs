use std::collections::HashSet;

fn main() {
	let s1 = HashSet::from([1,2,3]);
	let s2 = HashSet::from([2,3,4,5]);
	
	let result = merge(s1, &s2);
	
	println!("{:?}", result);
}

pub fn merge(mut s1: HashSet<i32>, s2: &HashSet<i32>) -> HashSet<i32> {

    for item in s2 {
        if *item % 2 == 0 {
            s1.insert(*item);
        }
    }

    s1
}