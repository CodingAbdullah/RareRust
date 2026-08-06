fn main() {
	let v = vec![4,8,14];
	let k = 8;
	let result = filter_lt_k(&v, k);
	println!("{:?}", result);
}

// pub fn filter_lt_k
pub fn filter_lt_k(v1: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..v1.len() {
        if v1[i] >= k {
            result.push(v1[i]);
        }
    }

    result
}