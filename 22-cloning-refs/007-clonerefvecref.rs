fn main() {
	let v = &vec![&1, &2, &3];
	
	let c: Vec<&i32> = v.clone();
	do_nothing(c);
}

fn do_nothing(v: Vec<&i32>) {}