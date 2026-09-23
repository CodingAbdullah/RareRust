use std::collections::HashSet;

fn main() {
    let v1 = vec![1, 2, 3, 2, 1];
    let v2 = vec![3, 1, 2, 3];
    println!("Same unique elements: {}", same_elements(&v1, &v2));
    
    let v3 = vec![1, 2, 3];
    let v4 = vec![1, 2, 4];
    println!("Same unique elements: {}", same_elements(&v3, &v4));
}

pub fn same_elements(v1: &[i32], v2: &[i32]) -> bool {
    // your code here: convert v1 to a HashSet<i32>
    // hint: use iter().copied().collect()
    let set1: HashSet<i32> = v1.iter().copied().collect();
    
    // your code here: convert v2 to a HashSet<i32>
    let set2: HashSet<i32> = v2.iter().copied().collect();
    
    // your code here: compare the sets
    set1 == set2
}