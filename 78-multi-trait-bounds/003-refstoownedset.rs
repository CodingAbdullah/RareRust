use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum E {
    A,
    B,
    C,
}

pub fn to_set<T: Copy + Eq + Hash>(v: &[T]) -> HashSet<T> {
    v.into_iter().copied().collect()
}

fn main() {
    let v = vec![E::A, E::C, E::C, E::B];
    let result = to_set(&v);
    println!("{:?}", result);
}