fn main() {
	let v = vec![10, 20, 30, 40, 50];
	let k1 = 30;
	let k2 = 60;

	println!("Index of {}: {:?}", k1, find_k_index(&v, k1));
	println!("Index of {}: {:?}", k2, find_k_index(&v, k2));
	println!("Index of {} in empty: {:?}", k1, find_k_index(&Vec::<i32>::new(), k1));
}

// pub fn find_k_index(v: &Vec<i32>, k: i32) your code here  
pub fn find_k_index(v: &Vec<i32>, k: i32) -> Option<i32> {
    let mut flag = None;

    for i in 0..v.len() {
        if v[i] == k {
            flag = Some(i as i32);
        }
    }

    return flag;
}   