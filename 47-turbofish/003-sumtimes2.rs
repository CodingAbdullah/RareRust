fn main() {
    let a = [1, 2, 3, 4, 5];
    let result = sum_x2(&a);
    println!("Sum × 2 = {}", result);
}

pub fn sum_x2(s: &[i32]) -> i32 {
    s.iter().sum::<i32>() * 2
}
