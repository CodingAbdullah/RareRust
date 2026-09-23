fn main() {
	let v = vec![2,3,5,7,11];
	
	for (i, val) in v.iter().enumerate() {
		println!("{} {}", i, val);
		accept(i, val);
	}
}

fn accept(_i: usize, _v: &i32) {}