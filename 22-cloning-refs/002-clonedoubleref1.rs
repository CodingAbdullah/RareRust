fn main() {

	let v = vec![1,2,3];
	let ref_ref_v = &&v;
	
	// clone "removes" one & and creates a &Vec<i32> not a Vec<i32>
	let v: Vec<i32> = (*ref_ref_v).clone();
}