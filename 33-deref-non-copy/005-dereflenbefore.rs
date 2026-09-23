use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let v = vec![1,2,3];
    let a = HashSet::from([17, 18, 19]);
    let b = HashMap::from([(100, 10), (50, 5), (20, 1)]);
    
    let result = all_same(&v, &a, &b);
    println!("{}", result);
}

pub fn all_same(v: &Vec<i32>, a: &HashSet<i32>, b: &HashMap<i32, i32>) -> bool {
    // your code here

    if (*v).len() == (*a).len() && (*v).len() == (*b).len() {
        return true;
    }
    else {
        return false;
    }
}