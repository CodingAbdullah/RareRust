use std::collections::HashSet;
fn main() {
    let a = [1, 0, 4];
    let set: HashSet<usize> = HashSet::from([1, 2]);

    let result = remove_if_idx_in_set(&a, &set);
    println!("{:?}", result);
}

pub fn remove_if_idx_in_set(arr: &[i32], set: &HashSet<usize>) -> Vec<i32> {
    // your code here
    arr.iter().enumerate().filter(
        | (x, _item) | { !set.contains(x) }).map(| (_, &item) | { item } ).collect()
}
