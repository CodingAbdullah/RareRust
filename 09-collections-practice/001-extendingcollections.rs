use std::collections::HashSet;

fn main() {
    let mut s1 = HashSet::from([1,2,3]);
    let s2 = HashSet::from([3,4,5]);
    
    let mut v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    
    println!("s1 before: {:?}", s1);
    s1.extend(s2);
    println!("s1 after extend: {:?}", s1);

    println!("v1 before: {:?}", v1);
    // YOUR CODE HERE extend v1 with v2 here
    v1.extend(v2);
    println!("v1 after extend: {:?}", v1);

} 