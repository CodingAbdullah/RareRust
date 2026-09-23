use std::collections::HashSet;

pub fn from_vector_to_set(values: Vec<i32>) -> HashSet<i32> {
    // your code here
    let vec_set: HashSet<i32> = values.into_iter().collect();

    return vec_set;
}

fn main() {
    let values = vec![3, 7, 3, 2, 7, 9];
    let result = from_vector_to_set(values);

    println!("{:?}", result);
}
