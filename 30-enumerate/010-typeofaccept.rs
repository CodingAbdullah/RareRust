fn main() {
	let v = vec![&1,&2,&3,&4];
	
	for (i, e) in v.iter().enumerate() {
	    accept(i, e);
	}
}

fn accept(teee: usize, eee: &&i32) {}