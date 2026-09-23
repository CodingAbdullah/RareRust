fn main() {
	let a: [Vec<i32>; 3] = [vec![1, 2], vec![2], vec![3]];
	take(a.clone());
	println!("a not consumed {:?}", a);
}

pub fn take(_a: [Vec<i32>; 3]) {}