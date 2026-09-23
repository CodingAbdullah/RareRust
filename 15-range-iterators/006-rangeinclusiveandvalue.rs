pub fn collect_and_sum_range(start: i32, end: i32) -> Vec<i32> {
    // your code here
    let mut vec: Vec<i32> = Vec::new();

    let mut sum = 0;

    for i in start..=end {
        sum = sum + i;
        vec.push(i);
    }

    vec.push(sum);

    vec
}

fn main() {
    // Example from description for general inclusive range
    for i in 1..=5 {
        println!("{}", i);
    }

    let result = collect_and_sum_range(4, 8);
    println!("{:?}", result); // Expected: [4, 5, 6, 7, 8, 30]
}
