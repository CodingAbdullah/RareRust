fn main() {
	let v = vec![1,2,3];
	let filter_even = true;
	
	let result = filter_even_odd(&v, filter_even);
	println!("{:?}", &v);
	println!("{}", filter_even);
	println!("{:?}", &result);
}

// pub fn filter_even_odd vector, filter_even... your code here
pub fn filter_even_odd(v: &Vec<i32>, filter: bool) -> Vec<i32> {
    if v.len() == 0 {
        return Vec::new();
    }

    let mut new_vec = Vec::new();

    if filter {
        for i in 0..v.len() {
            if v[i] % 2 != 0 {
                new_vec.push(v[i]);
            }
        }
    }
    else {
        for i in 0..v.len() {
            if v[i] % 2 == 0 {
                new_vec.push(v[i]);
            }
        }
    }
    
    return new_vec;
}