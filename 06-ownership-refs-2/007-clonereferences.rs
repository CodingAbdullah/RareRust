fn main() {
	let v = vec![1,2,3];
	
	let result = append_sum(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn append_sum(v: &Vec<i32>) -> Vec<i32> {
	let mut sum = 0;
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	// create a mutable vector my_v which is a clone of v on this line
    let mut my_v = v.clone();
	my_v.push(sum);
	my_v
} 