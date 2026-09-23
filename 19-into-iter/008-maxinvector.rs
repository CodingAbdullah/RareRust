fn main() {
	let v = vec![1,2,3];
	
	let result = get_max(&v);
	println!("{:?}", v);
	println!("{}", result);
}

pub fn get_max(v: &Vec<i32>) -> i32 {

	let mut max = v[0];

    for i in 1..v.len(){
        if v[i] > max {
            max = v[i];
        }
    }

	// TODO
	max
} 