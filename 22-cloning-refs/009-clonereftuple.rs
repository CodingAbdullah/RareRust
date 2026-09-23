fn main() {
	let t = &(10, 20);
	
	let c: (i32, i32) = *t; // edit this
	do_nothing(c);
}

fn do_nothing(_v: (i32, i32)) {}