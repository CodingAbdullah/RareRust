fn main() {
	let v = vec![1,2,3];
	
	for e in &v{
	    accept(e);
	}
}

fn accept(_e: &i32) {}