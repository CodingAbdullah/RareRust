use std::collections::HashMap;

// pub fn split_keys_values

pub fn split_keys_values(hm: HashMap<i32, i32>) -> (Vec<i32>, Vec<i32>) {
    (hm.keys().copied().collect(), hm.values().copied().collect())
}

fn main() {
    let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 30);

    let (keys, values) = split_keys_values(data);
    println!("Keys: {:?}", keys);     // [1, 2, 3]
    println!("Values: {:?}", values); // [10, 20, 30]
}