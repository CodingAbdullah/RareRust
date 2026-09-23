use std::collections::HashMap;

fn main() {
    let pairs: [(i32, i32); 3] = [(1, 10), (2, 20), (3, 30)];
    let _m: HashMap<i32, i32> = HashMap::from(pairs);
}
