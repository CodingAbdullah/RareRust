use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(3);
    
    let mut set_ref = &set; // Create a reference
    let mut set2 = set_ref.clone(); // YOUR CODE HERE clone set_ref here
    set2.insert(4);
    
    // set is unchanged because set2 is a clone
    println!("{:?}", set);
    println!("{:?}", set2);
} 