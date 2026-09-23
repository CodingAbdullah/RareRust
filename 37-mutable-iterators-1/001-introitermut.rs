fn main() {
	let mut v = vec![1,2,3];
	
	for e in v.iter_mut() {
        *e = *e + 1;
	}
	
	println!("{:?}", v);
}