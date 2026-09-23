use std::collections::HashSet;

fn main() {
	let v = vec![10, 11, 12, 10, 14];
	let result = index_of_first_duplicate(v);
	println!("{:?}", result); 
}

pub fn index_of_first_duplicate(v: Vec<i32>) -> Option<usize> {
	// your code here
    let mut hset: HashSet<i32> = HashSet::new();

    for (i, value) in v.iter().enumerate() {
        if hset.contains(value) {
            return Some(i);
        }
        else {
            hset.insert(*value);
        }
    }

    return None;
}