fn main() {
	let mut v = vec![vec![1], vec![2], vec![3]];

	append_0_to_all(&mut v);
	
	println!("{:?}", v);
}

pub fn append_0_to_all(v: &mut Vec<Vec<i32>>) {
    
    // your code here
    for vt in v {
        (*vt).push(0);
    }
}