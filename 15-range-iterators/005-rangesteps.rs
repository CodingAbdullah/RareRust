use std::collections::HashSet;

pub fn range_step_to_30(step: i32) -> HashSet<i32> {
    (0..30).step_by(step as usize).collect()
}

fn main() {
    let v: Vec<i32> = (0..10).step_by(2).collect(); // Example from description
    println!("{:?}", v); // [0, 2, 4, 6, 8]

    let result = range_step_to_30(5);
    println!("{:?}", result); // Expected: {0, 5, 10, 15, 20, 25}
}
