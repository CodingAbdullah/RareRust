use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();
	set.insert(5);
	set.insert(7);
	let v1 = vec![3,5,9]; // Contains 5
	let v2 = vec![1,2,3]; // No common elements
	
	let result1 = exists_in_set(&set, &v1);
	println!("Set {:?} has element from {:?}? {}", set, v1, result1);
	println!("{:?}", v1);
	println!("{:?}", set);

	let result2 = exists_in_set(&set, &v2);
	println!("Set {:?} has element from {:?}? {}", set, v2, result2);
}

// Check if any element in vector v exists in the set
pub fn exists_in_set(set: &HashSet<i32>, v: &Vec<i32>) -> bool {
	// your code here
    for i in 0..v.len() {
        if set.contains(&v[i]) {
            return true;
        }
    }
    return false;
} 