pub fn odd_descending(end: i32) -> Vec<i32> {
    // You code here
    let vec: Vec<i32> = (1..end).step_by(2).rev().collect();

    vec
}

fn main() {
    let result = odd_descending(20);
    println!("{:?}", result); // Expected: [19, 17, 15, 13, 11, 9, 7, 5, 3, 1]
}
