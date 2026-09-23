use std::collections::HashSet;

fn main() {
    let v = vec![1,2];
    let s = HashSet::from([1,2,3,4]);
    let result = remove_from_set(&v, s);
    println!("{:?}", result); // {3,4} or {4,3}
}

pub fn remove_from_set(v: &Vec<i32>, mut s: HashSet<i32>) -> HashSet<i32> {

    for value in v.iter() {
        s.remove(value);
    }
    s
}