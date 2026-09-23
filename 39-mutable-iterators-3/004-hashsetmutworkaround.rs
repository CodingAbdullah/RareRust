use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let mut vecset: Vec<i32> = set.iter().copied().collect();

    for item in vecset.iter_mut() {
        *item *= 2; // equivalent to *e = *e * 2;
    }

    let newset: HashSet<i32> = vecset.iter().copied().collect();
    
    println!("{:?}", newset);
}