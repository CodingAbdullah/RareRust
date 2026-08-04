fn main() {
	let v = vec![4,2,6];
	let result = append_sum(v);
	println!("{:?}", result);
}

pub fn append_sum(v: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut sum = 0;
    let mut new_vec = vec![];

    if v.len() == 0 {
        return vec![0]
    }
    else {
        for i in 0..v.len() {
            sum = sum + v[i];
            new_vec.push(v[i]);
        }

        new_vec.push(sum);
    }

    return new_vec;
} 