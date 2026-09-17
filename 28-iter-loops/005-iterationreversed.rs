fn main() {
	let v = vec![1,2,3,4];
		
	for e in v.iter().rev() {
	    println!("{}", e);
	}
	
	println!("not consumed: {:?}", v);
}