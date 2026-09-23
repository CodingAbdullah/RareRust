use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, 10), (2, 20), (3, 90)]);
    
    let value = get_or_zero(&map, 1);
    println!("Value for key 1: {}", value); // Should print 10

    let value = get_or_zero(&map, 4);
    println!("Value for key 4: {}", value); // Should print 0
}

// HashMap Get or Zero
pub fn get_or_zero(map: &HashMap<i32, i32>, key: i32) -> i32 {

    match map.get(&key) {
        Some(&value) => value,
        None => 0
    }
}