fn main() {
	let v = vec![1,5,8,12,3];
	let k = 2;
	let t = 4;
	
	let result = at_least_k_larger_than_t(v, k, t);
	println!("{}", result);
}

pub fn at_least_k_larger_than_t(v: Vec<i32>, k: usize, t: i32) -> bool {
    // your code here
    let mut count = 0;

    for i in 0..v.len() {
        if v[i] > t {
            count = count + 1;
        }
    }

    if count >= k {
        return true;
    }
    else {
        return false;
    }
} 