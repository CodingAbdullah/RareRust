fn main() {
	let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = v.clone(); // clone v here
    
    my_vec.push(6);
    my_vec
} 