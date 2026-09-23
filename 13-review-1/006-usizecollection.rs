// pub fn your code here
pub fn count_even_numbers(numbers: &Vec<i32>) -> (usize, usize) {
    let mut even_numbers = 0;

    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            even_numbers = even_numbers + 1;
        }
    }

    return (even_numbers as usize, numbers.len());
}
fn main() {
    let numbers = vec![2, 5, 8, 9, 12, 15];
    let result = count_even_numbers(&numbers);
    println!("Even numbers: {}, Length of vector: {}", result.0, result.1);
    // Output: Even numbers: 3, Length of vector: 6
}