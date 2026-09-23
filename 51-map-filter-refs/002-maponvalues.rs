use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(0, 1), (2, 3)]);
    let result = sum_values_inc(map);
    println!("{}", result);
}

pub fn sum_values_inc(map: HashMap<i32, i32>) -> i32 {
    // your code here
    map.values().map(|x| { *x + 1 }).sum()
}
