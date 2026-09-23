use std::collections::HashSet;

fn main() {
    let v = vec![1, 2, 2, 3, 1, 4, 5, 4];
    println!("Original vector: {:?}", v);
    // v is moved (consumed) here
    let set = vector_to_set(v);
    println!("Resulting set: {:?}", set);
    // Cannot use v after this point, e.g., println!("{:?}", v); would fail
}

// Function consumes the vector (takes ownership)
pub fn vector_to_set(v: Vec<i32>) -> HashSet<i32> {
    let mut vectorset = HashSet::new();

    for i in 0..v.len() {
        vectorset.insert(v[i]);
    }

    return vectorset;
} 