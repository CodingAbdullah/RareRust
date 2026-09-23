fn main() {

	// vector
	let a = [1, 2, 3];
	let slice_1 = &a[..];
	let v = Vec::from(slice_1);

	// string
	let slice_2: &str = "hello RareCode";
	let s = String::from(slice_2);
	
	println!("{}", s);
}
