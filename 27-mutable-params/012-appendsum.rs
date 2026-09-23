fn main() {
	let v: Vec<i32> = vec![1, 2, 3];
	
	let result = append_sum(&v);
	println!("{:?}", result);
}

// your code here
pub fn append_sum(vec: &Vec<i32>) -> Vec<i32> {
    let sum = vec.iter().sum();
    let mut new_vec = vec.clone();

    new_vec.push(sum);
    new_vec
}