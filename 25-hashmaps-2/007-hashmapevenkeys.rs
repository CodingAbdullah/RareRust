use std::collections::HashMap;

// pub fn even_keys_only
pub fn even_keys_only(hm: HashMap<i32, i32>) -> Vec<i32> {
    let mut nvec: Vec<i32> = Vec::new();

    for (key, _value) in hm.iter() {
        if *key % 2 == 0 {
            nvec.push(*key);
        }
    }

    nvec
    


}



fn main() {
    let mut data = HashMap::new();
    data.insert(1, 100);
    data.insert(2, 200);
    data.insert(3, 300);
    data.insert(4, 400);

    let filtered = even_keys_only(data);
    println!("{:?}", filtered); // [2, 4]
}
