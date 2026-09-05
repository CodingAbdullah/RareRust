fn main() {
	let v = vec![1,2,3];
	let a = 2;
	
	let result = increment_by(&v, a);
	
	println!("{}", a);
	println!("{:?}", &v);
	println!("{:?}", &result);
}

// pub fn increment_by vector and a... your code here
pub fn increment_by(v1: &Vec<i32>, a: i32) -> Vec<i32> {
    if v1.len() == 0 {
        return Vec::new();
    }
    else {
        let mut vec2 = v1.clone();

        for i in 0..vec2.len() {
            vec2[i] = vec2[i] + a;
        }

        return vec2;
    }
}