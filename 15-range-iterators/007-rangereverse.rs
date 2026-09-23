pub fn reverse_range(start: i32, end: i32) -> Vec<i32> {
    // Your code here

    let vec: Vec<i32> = (start..=end).rev().collect();

    vec
}

fn main() {
    // Example from description
    let v1: Vec<i32> = (8..=4).rev().collect(); // ❌ Wrong
    let v2: Vec<i32> = (4..=8).rev().collect(); // ✅ Correct
    println!("V1: {:?}, V2: {:?} ", v1, v2); // V1: [], V2: [8, 7, 6, 5, 4]

    let result = reverse_range(2, 7);
    println!("{:?}", result); // Expected by example: [7, 6, 5, 4, 3, 2]
}
