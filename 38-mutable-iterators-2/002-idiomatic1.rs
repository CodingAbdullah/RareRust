fn main() {
	let mut v = vec![7,8,9];
	
	for e in v.iter_mut() {
	    *e = inc_if_odd(e);
	}
	println!("{:?}", v);
}

pub fn inc_if_odd(e: &i32) -> i32 {
    if e % 2 == 1 {
        return e + 1;
    }
    *e
}