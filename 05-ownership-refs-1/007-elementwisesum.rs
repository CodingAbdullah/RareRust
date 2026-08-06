fn main() {
	let v1 = vec![1,2,3];
	let v2 = vec![1,1,2];
	
	let result = elementwise_sum(&v1,&v2);
	
	// can print because v1 and v2 are not consumed
	println!("{:?}", v1);
	println!("{:?}", v2);
	println!("{:?}", result);
}

pub fn elementwise_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..v1.len() {
        result.push(v1[i] + v2[i]);
    }

    return result;
} 