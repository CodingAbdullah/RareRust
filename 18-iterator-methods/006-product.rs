use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2, 3]);

    let result = product_of_set(s);
    println!("{}", result);
}

pub fn product_of_set(s: HashSet<i32>) -> i32 {
    // Your code here
    s.into_iter().product() // add .product() to the end
}
