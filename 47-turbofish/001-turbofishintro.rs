
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = sum_of_slice(&numbers);
    println!("{}", result);
}

fn sum_of_slice(slice: &[i32]) -> bool {
    if slice.iter().sum::<i32>() > 10 {
        return true;
    }
    false
}