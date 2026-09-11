use std::collections::HashSet;

fn main() {
    let v1 = vec![1, 2, 2, 3, 4, 4, 4, 5, 1];
    println!("Vector {:?} has {} duplicate items.", v1, num_duplicates(v1.clone()));
    
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector {:?} has {} duplicate items.", v2, num_duplicates(v2.clone()));

    let v3 = vec![7, 7, 7];
    println!("Vector {:?} has {} duplicate items.", v3, num_duplicates(v3.clone()));
}

pub fn num_duplicates(v: Vec<i32>) -> usize {
    // your code here
    let v_clone = v.clone();
    let v_clone_length = &v_clone.len();
    let setlength: HashSet<i32> = v_clone.into_iter().collect();
    return v_clone_length - setlength.len();

} 