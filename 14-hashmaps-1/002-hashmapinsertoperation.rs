use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, bool> = HashMap::new();
    hm.insert(2, true);
    hm.insert(3, false);
    hm.insert(4, true);
    hm.insert(5, false);
    // your code 

    println!("{:?}", hm); // Expected output based on description: {2: true, 3: false, 4: true, 5: false}
}
