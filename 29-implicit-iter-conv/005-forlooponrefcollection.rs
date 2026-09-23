fn main() {
	let v = vec![1,2,3];
	let w = &v;
	
	for e in w {
		accept(e);
	}
}

// what function signature should accept have?
fn accept(eee: &i32) {}