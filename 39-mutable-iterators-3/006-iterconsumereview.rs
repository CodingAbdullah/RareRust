use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    add_15(&mut map);
    
    println!("{:?}", map);
}

pub fn add_15(m: &mut HashMap<i32, i32>) {
    for (_k, v) in m.iter_mut() {
        *v += 15;
    }
}