use std::collections::HashMap;

fn main() {
    let v = vec![10,11,12];
    
    let hm: HashMap<usize, i32> = v.into_iter().enumerate().collect();
    println!("{:?}", hm);
}