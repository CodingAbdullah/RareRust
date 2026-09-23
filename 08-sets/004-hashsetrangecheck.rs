use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();
	set.insert(3);
	set.insert(5);
	let n = 6;
	
	let result = has_zero_to_n(&set, n);
	println!("Set {:?} has 0..{}? {}", set, n, result);

	let n2 = 3; // Set does not contain 0, 1, or 2
	let result2 = has_zero_to_n(&set, n2);
	println!("Set {:?} has 0..{}? {}", set, n2, result2);
}

pub fn has_zero_to_n(set: &HashSet<i32>, n: i32) -> bool {
	// your code here
    for item in set {
        if *item >= 0 && *item < n {
            return true;
        }
    }

    return false;
} 