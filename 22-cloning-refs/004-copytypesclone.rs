fn main() {
	let a = 2;
	let ref_a = &a;
	
	let _deref_a: i32 = ref_a.clone();
	println!("{}", "ok!");
}