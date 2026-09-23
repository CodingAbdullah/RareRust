use std::collections::HashMap;

fn main() {
    
    let hm = HashMap::from([(1,2),(2,3),(3,4)]);
    
    for (key, value) in hm.iter() {
        println!("{} {}", key, value);
    }
    
    println!("{:?}", hm); // hashmap is consumed;
}