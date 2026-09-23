use std::collections::HashMap;

fn main() {
    let map: HashMap<i32, i32> = HashMap::from([(1, 2), (3, 4)]);
    let result = sum_key_values_pairs(map);
    println!("{}", result); // 10 = 1 + 2 + 3 + 4

}

pub fn sum_key_values_pairs(map: HashMap<i32, i32>) -> i32 {
    map.iter().map(|(&key, &value)| { key + value }).sum()
}
