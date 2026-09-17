fn main() {
	let v = vec![1,2,3,4];

	for e in v.iter().step_by(2) {
	    println!("{}", e);   
	}
	
	println!("{:?}", v);
}