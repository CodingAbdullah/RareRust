use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, bool> = HashMap::new();
    // your code 
    hm.insert(1, false);
    hm.insert(2, false);
    hm.insert(1, true);

    println!("{:?}", hm);
}
