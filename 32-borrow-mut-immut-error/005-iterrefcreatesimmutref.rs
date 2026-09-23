use std::collections::HashMap;

fn main() {
    let mut s: HashMap<i32, i32> = HashMap::from([(1, 10), (2, 20)]);
    
    s.insert(3, 30);

    let keys_iter = s.keys();
    
    let keys_vec: Vec<i32> = keys_iter.copied().collect();
    println!("{:?}", keys_vec);
}