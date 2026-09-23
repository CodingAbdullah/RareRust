use std::collections::HashSet;

fn main() {
    let my_ref_set: HashSet<&i32> = HashSet::from([&1, &2, &3]);

    let my_owned_set: HashSet<i32> = my_ref_set.into_iter().copied().collect();
    foo(my_owned_set);
}

pub fn foo(_hs: HashSet<i32>) {}
