use std::collections::HashMap;

fn main() {
    let m = HashMap::from([(1,2),(3,4),(5,1)]);
    
    let result = prod_values(&m);
    println!("{}", result);
}

pub fn prod_values(m: &HashMap<i32, i32>) -> i32 {
    let mut i = 1;

    for value in m.values() {
        i *= *value;
    }
    i
}