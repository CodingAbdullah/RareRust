fn main() {
	let mut v = vec![1,2,3];
	
	append_zero(&mut v);
	append_zero(&mut v);
	append_zero(&mut v);
	// your code here
    append_zero(&mut v);

    append_zero(&mut v);
	
	println!("{:?}", v);
}

pub fn append_zero(v: &mut Vec<i32>) {
    v.push(0);
}