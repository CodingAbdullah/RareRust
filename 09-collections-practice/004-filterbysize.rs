use std::collections::HashSet;

fn main() {
    let v1 = vec![HashSet::from([1,2,3,4]), HashSet::from([1,2]), HashSet::from([5,6,7])]; 
    let k1 = 3;
    let result1 = remove_smaller_than_k(&v1, k1);
    println!("Original: {:?}, k={}, Result: {:?}", v1, k1, result1);

    let v2 = vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([3])];
    let k2 = 2;
    let result2 = remove_smaller_than_k(&v2, k2);
    println!("Original: {:?}, k={}, Result: {:?}", v2, k2, result2);
}

pub fn remove_smaller_than_k(v: &Vec<HashSet<i32>>, k: usize) -> Vec<HashSet<i32>> {
    // your code here
    let mut newvec: Vec<HashSet<i32>> = Vec::new();
    for i in 0..v.len() {
        if v[i].len() >= k {
            let newset = v[i].clone();
            newvec.push(newset);
        }
    }

    return newvec;
} 