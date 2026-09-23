use std::collections::HashSet;

fn main() {
    let hs = &HashSet::from([1,2,3]);
    
    let result = has(hs, 3);
    println!("{}", result);
}

pub fn has(hs: &HashSet<i32>, k: i32) -> bool {
    // your code here
    if (*hs).contains(&k){
        return true;
    }
    else {
        return false;
    }
}