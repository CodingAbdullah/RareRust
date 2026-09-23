fn main() {
	let mut v = vec![1,2,3];
	
	// your code here. Double each element such that the vector becomes 2, 4, 6
	// use iter_mut
    for e in v.iter_mut() {
        *e = *e * 2;
    }
	
	println!("{:?}", v);
}