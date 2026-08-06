fn main() {
	let v = vec![4,5,6];
	
	let v_sum = vec_sum(v); // v consumed here
	
	println!("{}", v_sum);
	// println!("{:?}", v);
}

pub fn vec_sum(v: Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 