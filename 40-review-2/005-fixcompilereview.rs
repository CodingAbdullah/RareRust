use std::collections::HashMap;

fn main() {
    let m = &HashMap::from([(1,2),(3,4),(5,1)]);
    
    for (k, v) in m {
        accept(k, v);
    }
}

pub fn accept(key: &i32, v: &i32) {}