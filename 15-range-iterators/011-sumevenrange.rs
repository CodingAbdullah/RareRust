use std::ops::Range;

pub fn sum_even_range(my_range: Range<i32>) -> i32 {
    // Your code here

    let mut sum = 0;

    for i in my_range {
        if i % 2 == 0 {
            sum = sum + i;
        }
    }

    sum
}

fn main() {
    let total = sum_even_range(0..5); // 0, 1, 2, 3, 4. Even: 0, 2, 4. Sum = 6
    println!("Sum: {}", total); // Expected: 6
}
