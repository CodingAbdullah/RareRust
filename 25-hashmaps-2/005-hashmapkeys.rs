use std::collections::HashMap;

pub fn list_keys(hm: HashMap<i32, i32>) -> Vec<i32> {
    hm.keys().copied().collect()
}



fn main() {
    let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 30);
    
    let keys = list_keys(data);
    println!("{:?}", keys); // [1, 2, 3]
}