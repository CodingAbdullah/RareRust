fn main() {
	let a = 2;
	let ref_ref_a = &&a;
	
	do_nothing(**ref_ref_a); // fix this line
}

fn do_nothing(a: i32) {

}