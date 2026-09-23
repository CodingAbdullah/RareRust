use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum E {
    A,
    B,
    C,
}

pub fn create_hash<T: Hash + Eq, U: Copy>(k: Vec<T>, v: Vec<U>) -> HashMap<T, U> {
    // your code here
    let mut vec_map: HashMap<T, U> = HashMap::new();
    
    for (i, key) in k.into_iter().enumerate() {
        vec_map.insert(key, v[i]);
    }

    vec_map
}


fn main() {
    let k = vec![E::A, E::C, E::C, E::B];
    let v = vec![1, 3, 3, 2];
    let result = create_hash(k, v);
    println!("{:?}", result);
    
    let k = vec![E::A, E::C, E::C, E::B];
    let v = vec![1.0, 3.0, 3.0, 2.0];
    let result = create_hash(k, v);
    println!("{:?}", result);
}