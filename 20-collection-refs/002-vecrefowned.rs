fn main() {
	let v = vec![&1,&2,&3];
	
	let result = convert(v);
	
	println!("{:?}", result);
}

pub fn convert(v: Vec<&i32>) -> Vec<i32> {
    v.into_iter().copied().collect()
}