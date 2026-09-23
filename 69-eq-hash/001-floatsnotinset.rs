use std::collections::HashSet;

fn main() {
    // This won't compile because f64 doesn't implement Eq
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    println!("Number set: {:?}", set);
}