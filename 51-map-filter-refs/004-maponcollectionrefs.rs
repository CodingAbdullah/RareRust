use std::collections::HashSet;
fn main() {
    let mut set = HashSet::from([&1, &2, &3]);
    let result = sum_odd(&mut set);
    println!("{}", result);
}

pub fn sum_odd(set: &mut HashSet<&i32>) -> i32 {
    // you will need to make an additional change so `.sum()` works
    set.iter().filter(|x| logic(***x) ).map(|&&x| x ).sum()
}

pub fn logic(x: i32) -> bool {
    x % 2 != 0
}
