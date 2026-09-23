use std::collections::HashSet;

fn main() {
    let s1 = Vec::from([1, 2, 3]);
    let s2 = Vec::from([4, 5, 6]);
    let s3 = Vec::from([7]);
    
    // This won't compile - HashSet doesn't implement Hash
    let final_set = HashSet::from([s1, s2, s3]);
    
    println!("{:?}", final_set);
}