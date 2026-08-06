// Write your starter code here

fn main() {
	let x = vec![1,2,3];
	let y = &x;
	
	println!("{:?}", x);
	println!("{:?}", y);
	
	// fix this - add & to create a reference to x
	println!("{}", vec_sum(&x));
	
	// y is already a reference
	// so this works
	println!("{}", vec_sum(y));
}

pub fn vec_sum(v: &Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 