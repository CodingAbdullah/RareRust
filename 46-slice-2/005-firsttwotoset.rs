use std::collections::HashSet;
fn main() {
    let a = [&1,&2,&3,&4,&5];
    let result = first_two_as_set(&a);
    println!("{:?}", result);
}

pub fn first_two_as_set(slr: &[&i32]) -> Option<HashSet<i32>> {
    // your code here
    if slr.len() < 2 {
        return None;
    }
    else {
        return Some(HashSet::from([*slr[0], *slr[1]]));
    }
}