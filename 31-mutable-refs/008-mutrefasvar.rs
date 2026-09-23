fn main() {
	let mut v = vec![1,2,3];
	let mr = &mut v;
	
	accept(mr);
}

fn accept(_mr: &mut Vec<i32>) {}