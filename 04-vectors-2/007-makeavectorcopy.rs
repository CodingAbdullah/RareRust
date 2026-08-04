fn main() {
	let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = make_copy(v);
    
    // make the change
    my_vec.push(6);
    my_vec
}

pub fn make_copy(v: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut new_vec: Vec<i32> = vec![];

    if v.len() == 0 {
        return new_vec;
    }
    else {
        for i in 0..v.len() {
            new_vec.push(v[i]);
        }
    }

    return new_vec;
} 