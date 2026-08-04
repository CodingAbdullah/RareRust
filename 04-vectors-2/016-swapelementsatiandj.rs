fn main() {
	let v = vec![1,2,3,4];
	let i = 1;
	let j = 2;
	
	let result = swap_ij(v, i, j);
	println!("{:?}", result);
}

pub fn swap_ij(v: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
    // your code here
    let mut new_vec = vec![];

    for i in 0..v.len() {
        new_vec.push(v[i]);
    }

    if i >= v.len() || j >= v.len() || v.len() == 0 {
        return new_vec;
    }
    else {
        let temp = new_vec[i];
        new_vec[i] = new_vec[j];
        new_vec[j] = temp;

        return new_vec;  
    }
} 