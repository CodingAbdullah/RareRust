fn main() {
	let v = vec![1,2,3];
	let w = v;
	println!("{:?}", w); // ok
	// println!("{:?}", v); // w consumed v, so v cannot be used here
}