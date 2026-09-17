use std::collections::HashMap;

fn main() {
	let hm = HashMap::from([(1,2),(3,4),(5,6)]);
	
	for (k, v) in &hm {
	    accept1(k, v);
	}
	
	for e in &hm {
	    accept2(e);
	}
}

fn accept1(kay: &i32, vee: &i32) {}

fn accept2(eee: (&i32, &i32)) {}