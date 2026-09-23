use std::collections::HashMap;

fn main() {
    
    let hm = HashMap::from([(1,2),(2,3),(3,4)]);
    
    for (key, value) in hm.iter() {
        foo(key, value);
    }
    
    println!("{:?}", hm); // hashmap is not consumed;
}

pub fn foo(k: &i32, v: &i32) {}