fn main() {
	let v = vec![1,2,3];
	let k = 4;
	let result = find(&v, k);
	println!("{}", result);

	let k = 2;
	let result = find(&v, k);
	println!("{}", result);
}

pub fn find(v: &Vec<i32>, k: i32) -> i32 {
	for i in 0..v.len() {
	    if k == v[i] {
	        return i as i32; // add as i32
	    }
	}
	-1
} 