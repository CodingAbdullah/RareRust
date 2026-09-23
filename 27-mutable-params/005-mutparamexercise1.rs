fn main() {
	let v = vec![1,2,3];
	let idx = 1;
	let result = double_at(v, idx);
	println!("{:?}", result);
}

// pub fn double_at
pub fn double_at(mut v: Vec<i32>, idx: u32) -> Vec<i32> {
    if idx as usize > v.len() {
        return v;
    }
    else {
        if v.len() == 0 {
            return vec![];
        }
        v[idx as usize] = v[idx as usize]*2;
        v
    }
}