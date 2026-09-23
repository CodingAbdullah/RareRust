use std::collections::HashMap;

fn main() {
    let m = HashMap::from([(1,2),(3,4),(5,1)]);
    let result = values_as_vec(&m);
    println!("{:?}", result);
}

pub fn values_as_vec(m: &HashMap<i32, i32>) -> Vec<i32> {
    let newvec: Vec<i32> = m.values().copied().collect();
    newvec
}