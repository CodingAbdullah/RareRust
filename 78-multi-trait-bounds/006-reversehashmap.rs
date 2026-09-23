use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum E {
    A,
    B,
    C,
}

// your code here
pub fn reverse<T: Eq + Hash, U: Eq + Hash>(kv_map: HashMap<T, U>) -> HashMap<U, T> {
    let mut new_map: HashMap<U, T> = HashMap::new();

    for (k, v) in kv_map {
        new_map.insert(v, k);
    }

    new_map
}

fn main() {
    let v = HashMap::from([(E::A, 1), (E::C, 2), (E::C, 2)]);
    let result = reverse(v);
    println!("{:?}", result);
}