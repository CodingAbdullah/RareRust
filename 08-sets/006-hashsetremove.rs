use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    println!("Before remove: {:?}", set);
    set.remove(&10);
    set.remove(&9); // Removing non-existent item is ok
    // remove a nonexistent item here
    set.remove(&124);

    println!("After remove: {:?}", set);
    
    println!("Contains 10? {}", (&set).contains(&10));
    println!("Contains 8? {}", (&set).contains(&8));
} 