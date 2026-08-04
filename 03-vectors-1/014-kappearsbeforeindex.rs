fn main() {
	let v = vec![1,2,3,4,5];
	let k = 3;
	let idx = 4;
	
	let result = k_appears_before_idx(v, k, idx);
	println!("{}", result);
}

pub fn k_appears_before_idx(v: Vec<i32>, k: i32, idx: usize) -> bool {
    // your code here

    for i in 0..v.len() {
        if v[i] == k && i < idx {
            return true;
        }
    }

    return false;
} 