use std::fmt::Debug;

pub fn all_or_empty<T: Debug+ Clone>(v: Vec<T>) -> Vec<T> {
    if v.len() > 3 {
        return v.clone();
    }
    
    println!("{:?} is too short", v);
    Vec::new()
}

fn main() {
    let v = vec![1,2,3,4];
    let result = all_or_empty(v);
    
    println!("{:?}", result);
    
    let v = vec!["a".to_string(), "b".to_string()];
    let result = all_or_empty(v);
    
    println!("{:?}", result);
}