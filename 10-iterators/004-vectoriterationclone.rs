fn main() {
	let v = Vec::from([2,4,8,10]);
	
	// Clone and iter
	let v_clone = v.clone();
    let v_iter = v_clone.into_iter();
    
	for item in v_iter {
		println!("{}", item);
	}
	
	println!("{:?}", v);
} 