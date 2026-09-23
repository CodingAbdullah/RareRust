use std::collections::HashSet;

fn main() {
    let v = vec![1, 2, 3, 4];
    let set = HashSet::from([2, 3]); 
    
    let result = keep_elements_in_set(v, set);
    println!("{:?}", result);
}

pub fn keep_elements_in_set(v: Vec<i32>, s: HashSet<i32>) -> Vec<i32> {
    // your code here
    v.into_iter().filter(|&x| { s.contains(&x) }).collect()
}