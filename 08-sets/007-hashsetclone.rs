use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(3);
    
    let mut set2 = // clone set here
    // insert 4 into set2 here
    set.clone();
    set2.insert(4);
    println!("{:?}", set);
    println!("{:?}", set2);
} 