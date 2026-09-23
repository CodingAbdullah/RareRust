fn main() {
	let a = [1, 2, 3];
	let slice_1 = &a[..];
	let _v: Vec<i32> = slice_1.into();

	let slice_2 = "hello RareCode";
	let _s: String = slice_2.into();
	
	let _s2: String = String::from(slice_2); // your code here
}
