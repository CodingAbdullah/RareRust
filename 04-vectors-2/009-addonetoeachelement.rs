fn main() {
	let v = vec![4,2,6];
	let result = add_1_to_each(v);
	println!("{:?}", result);
}

// pub fn add_1_to_each your code here
pub fn add_1_to_each(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec![];

    if v.len() == 0 {
        return new_vec;
    }
    else {
        for i in 0..v.len() {
            new_vec.push(v[i]);
        }
    }

    for j in 0..new_vec.len() {
        new_vec[j] = new_vec[j] + 1;
    }

    return new_vec;
}