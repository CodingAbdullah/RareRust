fn main() {
    // v is mutable
     let mut v = vec![1,2,3];
     
     let result = sum(&v);
     println!("{}", result);
 }
 
 // note that sum doesn't mutate
 pub fn sum(v: &Vec<i32>) -> i32 {
     v.iter().sum()
 }