use std::collections::HashSet;

fn main() {
    let mut s1 = HashSet::new();
    s1.insert(1);
    s1.insert(2);
    s1.insert(3);
    
    let l = size_of_set(&s1);
    println!("{}", l);
}

pub fn size_of_set(s: &HashSet<i32>) -> usize {
    // your code here

    s.len()
} 