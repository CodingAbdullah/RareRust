fn main() {
	let v = vec![1,2,1,3,4];
	let result = first_unsorted(v);
	println!("{}", result);
}

// pub fn first_unsorted   your code here

pub fn first_unsorted(v: Vec<i32>) -> usize {

    if v.len() == 0 {
        return 0;
    }
    else if v.len() == 1 {
        return 0;
    }
    else {
        for i in 1..v.len() {
            if v[i-1] <= v[i] {
                continue;
            }
            else {
                return i;
            }
        }
    }
    0
}