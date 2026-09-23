use std::collections::HashSet;

fn main() {
    let mut s = HashSet::from([1,2,3]);
    
    let ref_s = &s;
    let result_1 = sum(ref_s);
    println!("{}", result_1);
    
    println!("{}", sum(ref_s));

    s.insert(4);
}

pub fn sum(s: &HashSet<i32>) -> i32 {
    s.iter().sum()
}