use std::collections::HashSet;
use std::ops::Range;

pub fn collect_range_values(r: Range<usize>) -> HashSet<usize> {
    // Your code here
    let setrange = r.into_iter().collect();

    setrange
}

fn main() {
    // Example from description
    let my_range: Range<i32> = 0..10;
    println!("{:?}", my_range);

    let values = collect_range_values(5..10);
    println!("{:?}", values); // Expected: {5, 6, 7, 8, 9}
}
