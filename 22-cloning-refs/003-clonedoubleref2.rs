fn main() {

	let v = vec![1,2,3];
	let ref_ref_v = &&v;
	
	let result: Vec<i32> = (*ref_ref_v).clone();
}