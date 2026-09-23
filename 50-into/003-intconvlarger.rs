fn main() {
    
    let v: Vec<i32> = [1, 2, 3].into();
    let result = upconvert(v);
    println!("{:?}", result);
}

pub fn upconvert(v: Vec<i32>) -> Vec<i64> {
    // your code here
    let newvec: Vec<i64>  = v.into_iter().map(|x| { x as i64 }).collect();
    newvec
}