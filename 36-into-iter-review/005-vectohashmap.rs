use std::collections::HashMap;

fn main() {
	let v = vec![0,1,2];
	
	let hm: HashMap<usize, &i32> = v.iter().enumerate().collect(); 
	println!("v not consumed: {:?}", v);
	println!("HashMap: {:?}", hm);
}