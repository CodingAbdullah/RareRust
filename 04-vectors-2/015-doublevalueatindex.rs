fn main() {
	let v = vec![1,2,3,4];
	let idx = 1;
	
	let result = double_at_idx(v, idx);
	println!("{:?}", result);
}

pub fn double_at_idx(v: Vec<i32>, idx: usize) -> Vec<i32> {
    // your code here
    let mut new_vec = vec![];

    for i in 0..v.len() {
        new_vec.push(v[i]);
    }

    if idx >= v.len() {
        return new_vec;
    }
    else {
        new_vec[idx] = new_vec[idx] * 2;
    }

    return new_vec;
} 