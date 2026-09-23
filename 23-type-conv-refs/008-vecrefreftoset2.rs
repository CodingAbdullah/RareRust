use std::collections::HashSet;

fn main() {
    let v: &Vec<&i32> = &vec![&1,&2,&3];

    let _s: HashSet<i32> = v.clone().iter().copied().copied().collect();
}