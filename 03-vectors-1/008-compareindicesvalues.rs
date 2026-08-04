fn main() {
	let i = 0;
	let j = 1;
	let v = vec![5,3,15,21];
	
	let result = index_greater(v, i, j);
	println!("{}", result);
}

// pub fn index_greater   your code here

pub fn index_greater(v: Vec<i32>, i: usize, j: usize) -> bool {
    if v[i] >= v[j] {
        return true;
    }
    else {
        return false;
    }
}