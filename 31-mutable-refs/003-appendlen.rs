fn main() {
	let mut v = vec![1,2,3]; // add mut here
	
	append_len(&mut v);
	println!("{:?}", v);
}

pub fn append_len(v: &mut Vec<i32>) {
    let length: i32 = v.len() as i32;

    v.push(length);
}