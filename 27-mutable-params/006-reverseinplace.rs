fn main() {
	let v = vec![1,2,3,4,5,6];
	let result = reverse_in_place(v);
	println!("{:?}", result);
}

pub fn reverse_in_place(mut v: Vec<i32>) -> Vec<i32> {
    let halfway_point = v.len()/2;
    let full_length = v.len();

    for i in 0..halfway_point {
        let temp = v[full_length - i - 1];
        v[full_length - i - 1] = v[i];
        v[i] = temp;
    }
    v
}