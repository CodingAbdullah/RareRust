fn main() {
    let a = [1,2,3,4,5];

    let result = sum_of_squares_tail(&a);
    println!("{}", result);
}

pub fn sum_of_squares_tail(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    else if arr.len() == 1 {
        return 0;
    }
    else {
        arr[1..].iter().map(|&element| { element*element }).sum::<i32>()
    }
    //eiyour code here
}