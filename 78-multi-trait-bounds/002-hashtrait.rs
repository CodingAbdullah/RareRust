use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum E {
    A,
    B,
    C,
}

pub fn to_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}

fn main() {
    let v = vec![E::A, E::C, E::C, E::B];
    let result = to_set(v);
    println!("{:?}", result);
}
