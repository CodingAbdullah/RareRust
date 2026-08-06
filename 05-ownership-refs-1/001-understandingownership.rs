fn main() {
	let v = vec![1,2,3];
	do_nothing(v); // the first `do_nothing` consumed v
	// do_nothing(v); // this `do_nothing` cannot use v since it was consumed. Commment it out.
}

pub fn do_nothing(v: Vec<i32>) -> Vec<i32> {
	v
} 