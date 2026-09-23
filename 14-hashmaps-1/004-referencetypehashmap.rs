use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    hm.insert(2, 5);
    hm.insert(4, 6);

    // First part of the problem: look up key 2
    // Second part of the problem: look up key 3 (doesn't exist)
    println!("{:?}", hm.get(&2));
    println!("{:?}", hm.get(&3));
}
