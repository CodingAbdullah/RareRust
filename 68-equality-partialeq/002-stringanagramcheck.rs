use std::collections::HashMap;

fn main() {
    let s1 = "listen";
    let s2 = "silent";
    println!("{} and {} are anagrams: {}", s1, s2, are_anagrams(s1, s2));
    
    let s3 = "hello";
    let s4 = "world";
    println!("{} and {} are anagrams: {}", s3, s4, are_anagrams(s3, s4));
}

pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut map1: HashMap<char, usize> = HashMap::new();
    let mut map2: HashMap<char, usize> = HashMap::new();
    
    for c in s1.chars() {
        if map1.get(&c).is_some() {
            let count = map1.get(&c).unwrap();
            map1.insert(c, count + 1);
        } 
        else {
            map1.insert(c, 1);
        }
    }
    
    for c in s2.chars() {
        if map2.get(&c).is_some() {
            let count = map2.get(&c).unwrap();
            map2.insert(c, count + 1);
        } 
        else {
            map2.insert(c, 1);
        }
    }
    
    // your code here: compare the two maps for equality
    map1 == map2
}