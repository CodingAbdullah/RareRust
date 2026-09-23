pub fn sum_and_cast(numbers: Vec<u32>) -> usize {
    // your code here
    let mut sum = 0;

    for i in 0..numbers.len() {
        sum = sum + numbers[i];
    }

    return sum as usize;
}

fn main() {
    let my_numbers = vec![10, 20, 30, 40, 50];
    let result = sum_and_cast(my_numbers);
    println!(
        "Sum as u32: {}, Sum as usize: {}",
        (10u32 + 20 + 30 + 40 + 50),
        result
    );
    // Example output assumes the sum is 150.
    // Output: Sum as u32: 150, Sum as usize: 150

    let empty_vec: Vec<u32> = vec![];
    let result_empty = sum_and_cast(empty_vec);
    println!("Sum as u32: 0, Sum as usize: {}", result_empty);
    // Output: Sum as u32: 0, Sum as usize: 0
}
