use std::collections::HashSet;

pub fn collect_range_to_set(start: i32, end: i32) -> HashSet<i32> {
    // Your code here
    let rangeset: HashSet<i32> = (start..end).collect();

    return rangeset;
}

fn main() {
    let v: HashSet<u32> = (0..10).collect(); // Example from description
    println!("{:?}", v);

    let result = collect_range_to_set(0, 10);
    println!("{:?}", result); // Expected: {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
}
