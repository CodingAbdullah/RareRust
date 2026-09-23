pub fn reverse_range_with_step(step_by: usize) -> Vec<i32> {
    // Your code here
    let vec: Vec<i32> = (0..=10).rev().step_by(step_by).collect();

    vec
}

fn main() {
    // Example from description
    let a: Vec<i32> = (1..9).step_by(3).rev().collect();
    println!("{:?}", a); // [7, 4, 1]

    let b: Vec<i32> = (1..9).rev().step_by(3).collect();
    println!("{:?}", b); // [8, 5, 2]

    println!("{:?}", reverse_range_with_step(2)); // Expected: [10, 8, 6, 4, 2, 0]
}
