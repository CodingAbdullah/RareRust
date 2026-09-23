use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    for v in map.values_mut() {
        *v = *v + 5;
    }
    
    println!("{:?}", map);
}