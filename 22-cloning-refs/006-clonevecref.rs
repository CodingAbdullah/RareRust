fn main() {

	let v = vec![&1,&2,&3];
	
	let result = v.clone();
	do_nothing(result);
}

fn do_nothing(v_vec: Vec<&i32>) {}