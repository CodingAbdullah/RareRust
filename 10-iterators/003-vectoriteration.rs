fn main() {
	let v = Vec::from([2,4,8,10]);
	
	// Create iter
	let v_iter = v.into_iter();
	for item in v_iter {
		println!("{}", item);
	}
} 