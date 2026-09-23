fn main() {
    let v = vec![5,6,7,8,9,10,11];
    
    let result = keep_less_than_ten(v);
    println!("{:?}", result);
}

pub fn keep_less_than_ten(v: Vec<i32>) -> Vec<i32> {
    // your code here
    v.into_iter().filter(|&x| { x < 10 }).collect()
}