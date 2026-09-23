fn main() {
	let mut v = vec![1,2,3];

	for e in (&mut v).into_iter() {
		*e = *e * 6;
	}

	println!("{:?}", v);
}