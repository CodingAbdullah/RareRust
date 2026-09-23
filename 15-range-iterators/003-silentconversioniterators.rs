pub fn collect_range_to_vector(start: usize, end: usize) -> Vec<usize> {
    // Your code here
    let vecrange: Vec<usize> = (start..end).collect();

    return vecrange;
}

fn main() {
    let v: Vec<usize> = (0..10).collect(); // Example from description
    println!("{:?}", v);

    let result = collect_range_to_vector(3, 7);
    println!("{:?}", result); // Expected: [3, 4, 5, 6]
}
