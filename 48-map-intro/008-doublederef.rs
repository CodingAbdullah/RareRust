fn main() {
    let v = vec![&&1,&&2,&&3];
    let result = convert_double_ref_to_owned(v);
    println!("{:?}", result); // Output: [1, 2, 3]   
}

pub fn convert_double_ref_to_owned(v: Vec<&&i32>) -> Vec<i32> {
    // your code here
    v.into_iter().map(|x| { ** x}).collect()
}