fn main() {
	let v = vec![1,2,3];
	let sum = vec_sum(&v); // change v to &v
	
	println!("{:?}", v);
	println!("{}", sum);
}

// change Vec<i32> to &Vec<i32>
pub fn vec_sum(v: &Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 