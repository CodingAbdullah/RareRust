use std::collections::HashSet;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum E {
    A,
    B,
    C,
}

// your code here has_duplicates
pub fn has_duplicates<T: Copy + Hash + Eq>(v: &[T]) -> bool {
    let v_set: HashSet<T> = v.iter().copied().collect();

    if v_set.len() == v.len() {
        return false;
    }
    else {
        return true;
    }
}

fn main() {
    let v = vec![1, 3, 3, 2, 2];
    let result = has_duplicates(&v);
    println!("{:?}", result);
    
    let v = vec![E::A, E::A];
    let result = has_duplicates(&v);
    println!("{:?}", result);
}