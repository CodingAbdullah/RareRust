use std::collections::HashMap;

fn main() {
    let a = [(1, 7), (2, 11), (3, 13)];
    
    let m: HashMap<i32, i32> = a.into();
    println!("{:?}", m);
}