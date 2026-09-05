fn main() {
	let k = 5;
	let v = vec![4,5,6,7];
	
	let result = is_in_vector(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
}

pub fn is_in_vector(v1: &Vec<i32>, k: i32) -> bool {
	// your code here
    let mut flag = false;

    for i in 0..v1.len() {
        if v1[i] == k {
            flag = true;
            break;
        }
    }
    flag
}