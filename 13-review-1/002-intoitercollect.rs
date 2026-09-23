use std::collections::HashSet;

pub fn from_set_to_vector(set: HashSet<i32>) -> Vec<i32> {
    // your code here
    let vec: Vec<i32> = set.into_iter().collect();
    
    return vec;
}

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(4);
    set.insert(7);

    let mut result = from_set_to_vector(set);
    result.sort(); // Sort for consistent output order in println
    println!("{:?}", result); // Example: [4, 7, 10]
}
