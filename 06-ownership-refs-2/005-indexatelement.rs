fn main() {
	let v = vec![1,2,3];
	let k = 3;
	
	let result = find_idx_of(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
}

// pub fn find_idx_of ... your code here
pub fn find_idx_of(v1: &Vec<i32>, k: i32) -> i32 {
    if v1.len() == 0 {
        return 0;
    }
    else {
        for i in 0..v1.len() {
            if v1[i] == k {
                return i as i32;
            }
        }

        return v1.len() as i32;
    }
}