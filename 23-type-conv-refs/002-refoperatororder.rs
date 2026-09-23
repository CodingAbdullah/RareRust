fn main() {
	let v: &Vec<i32> = &vec![1,2,3];
	let cv: &Vec<&i32> = &v.into_iter().collect();
}
