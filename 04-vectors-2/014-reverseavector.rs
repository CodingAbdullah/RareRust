fn main() {
	let v = vec![1,2,3,4];
	
	let result = reverse(v);
	println!("{:?}", result);
}

pub fn reverse(v: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut new_vec = vec![];

    for i in (0..v.len()).rev() {
        new_vec.push(v[i]);
    }

    return new_vec;
} 