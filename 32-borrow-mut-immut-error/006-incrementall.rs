fn main() {
	let mut v = vec![1,2,3];

	inc_all(&mut v); 
	println!("{:?}", v);
}

pub fn inc_all(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] = v[i] + 1;
    }
}