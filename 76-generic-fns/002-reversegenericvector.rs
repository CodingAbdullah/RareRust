// your code here
pub fn rev<T>(v: Vec<T>) -> Vec<T> {
    v.into_iter().rev().collect()
}
fn main() {
    let v = vec![1,2,3,4];
    let result = rev(v);
    println!("{:?}", result);
    
    let v = vec!['a', 'b', 'c', 'd'];
    let result = rev(v);
    println!("{:?}", result);
}