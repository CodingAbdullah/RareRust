fn main() {
	let v = vec![2,3,5,7,11];
	
	for t in v.iter().enumerate() {
		println!("{:?}", t);
		accept(t);
	}
}

fn accept(tee: (usize, &i32)) {}