use std::collections::HashSet;

fn main() {
    let s: HashSet<i32> = HashSet::from([1, 2, 3, 0, -5]);
    
    let result = inc_set(s);
    println!("{:?}", result);
}

pub fn inc_set(s: HashSet<i32>) -> HashSet<i32> {

    // your code here
    let s_clone_iter = s.clone().into_iter();
    let mut newset: HashSet<i32> = HashSet::new();

    for item in s_clone_iter {
        newset.insert(item + 1);
    }

    return newset;
} 