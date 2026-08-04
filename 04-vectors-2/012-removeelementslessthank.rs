fn main() {
	let v = vec![1,2,3,4];
	let k = 3;
	let result = remove_less_than_k(v, k);
	println!("{:?}", result);
}

// your code here
pub fn remove_less_than_k(v: Vec<i32>, k: i32) -> Vec<i32> {
    let mut new_vec = vec![];
    
    for i in 0..v.len() {
        if v[i] >= k {
            new_vec.push(v[i]);
        }
    }

    return new_vec;
}