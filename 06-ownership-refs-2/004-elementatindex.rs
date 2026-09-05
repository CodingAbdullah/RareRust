fn main() {
	let k = 5;
	let v = vec![4,5,6,7];
	let idx = 1;
	
	let result = k_is_at_idx(&v, k, idx);
	println!("{}", result);
	println!("{}", k);
	println!("{}", idx);
	println!("{:?}", &v);
}

pub fn k_is_at_idx(v: &Vec<i32>, k: i32, idx: i32) -> bool {
	// your code here
    if idx < 0 {
        return false;
    }
    
    let idx = idx as usize;

    if idx >= v.len() || v.len() == 0 {
        return false;
    }
    
    v[idx] == k
} 