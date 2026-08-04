fn main() {
	let v1 = vec![1,2,3,4];
	let v2 = vec![0,1,0,4];
	
	let result = elementwise_add(v1, v2);
	println!("{:?}", result);
}

pub fn elementwise_add(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut new_vec = vec![];

    if v1.len() == 0 {
        return new_vec;
    }
    else {
        for i in 0..v1.len() {
            new_vec.push(v1[i] + v2[i]);
        }
    }

    return new_vec;
} 