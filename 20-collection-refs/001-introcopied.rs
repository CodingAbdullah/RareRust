fn main() {
	let v = vec![1,2,3];
	
	let result = get_max(&v);
	
	println!("{:?}", result);
}

pub fn get_max(v: &Vec<i32>) -> Option<i32> {
    let maxx = v.into_iter().max();
    maxx.copied()
}