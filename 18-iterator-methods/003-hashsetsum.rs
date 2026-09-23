use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2, 3]);

    let result = sum_of_set(s);
    println!("{}", result);
}

pub fn sum_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    let sum = s.into_iter().sum();

    sum
}
