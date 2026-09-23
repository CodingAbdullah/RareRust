use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    let v_to_remove = vec![2, 4, 6]; // 6 is not in the set

    println!("Original set: {:?}", set);
    println!("Vector to remove: {:?}", v_to_remove);
    
    let result_set = remove_vector_elements(&set, &v_to_remove);
    
    println!("Result set: {:?}", result_set);
    println!("Original set unchanged: {:?}", set); // Show original is unchanged
}

pub fn remove_vector_elements(set: &HashSet<i32>, v: &Vec<i32>) -> HashSet<i32> {
    // your code 
    let mut cloneset = set.clone();

    for i in 0..v.len() {
        if cloneset.contains(&v[i]) {
            cloneset.remove(&v[i]);
        }
    }

    cloneset
    // clone the set as mutable, then remove the items in v, then return it the clone
} 