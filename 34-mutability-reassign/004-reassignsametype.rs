use std::collections::HashSet;

fn main() {
    
    let mut v = vec![1,2,3];
    println!("v is {:?}", v);
    
    let s: Vec<i32> = HashSet::from([&4, &5, &6]).iter().copied().copied().collect();
    
    v = s;
    println!("v is {:?}", v);
}