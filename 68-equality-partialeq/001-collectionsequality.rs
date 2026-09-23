use std::collections::{HashSet, HashMap};

fn main() {
    // Strings
    let str1 = "hello";
    let str2 = "hello";
    let str3 = "world";
    println!("str1 == str2: {}", str1 == str2);
    println!("str1 == str3: {}", str1 == str3);
    
    // Vectors
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let v3 = vec![3, 2, 1];
    println!("v1 == v2: {}", v1 == v2); 
    println!("v1 == v3: {}", v1 == v3); 
    
    // Sets (order doesn't matter for sets!)
    let s1 = HashSet::from([1, 2, 3]);
    let s2 = HashSet::from([3, 1, 2]);
    println!("s1 == s2: {}", s1 == s2); 
    
    // HashMaps (order doesn't matter!)
    let m1 = HashMap::from([(3, 4), (1, 2)]);
    let m2 = HashMap::from([(1, 2), (3, 4)]);
    println!("m1 == m2: {}", m1 == m2); 
}