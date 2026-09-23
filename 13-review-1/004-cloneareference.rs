pub fn clone_positive_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut positive_numbers = Vec::new();

		// Your logic here
        for i in 0..numbers.len() {
            if numbers[i] > 0 {
                positive_numbers.push(numbers[i]);
            }
        }

    positive_numbers
}


fn main() {
    let input_numbers = vec![3, -4, 10, 0, -7, 8];
    let positives = clone_positive_numbers(&input_numbers);
    println!("Positive numbers: {:?}", positives); // Output: [3, 10, 8]
}