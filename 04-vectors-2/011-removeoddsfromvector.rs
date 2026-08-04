fn main() {
	let v = vec![1,2,3,4];
	let result = remove_odd(v);
	println!("{:?}", result);
}

//pub fn remove_odd ... your code here.
pub fn remove_odd(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec![];

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            new_vec.push(v[i]);
        }
    }

    return new_vec;
}