use std::collections::HashSet;

fn main() {
    let hs: &HashSet<&i32> = &HashSet::from([&1, &2, &3]);

    let owned_hs = hs.clone().into_iter().copied().collect();
    do_nothing(owned_hs);
}

fn do_nothing(_hs: HashSet<i32>) {}
