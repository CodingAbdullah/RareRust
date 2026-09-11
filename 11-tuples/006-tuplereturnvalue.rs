pub fn min_and_max(a: i32, b: i32) -> (i32, i32) {
    // your code here
    if a > b {
        (b, a)
    }
    else {
        (a, b)
    }
}

fn main() {
    let result = min_and_max(5, 3);
    println!("Min: {}, Max: {}", result.0, result.1);
    let result2 = min_and_max(-5, 7);
    println!("Min: {}, Max: {}", result2.0, result2.1);
    let result3 = min_and_max(100, -100);
    println!("Min: {}, Max: {}", result3.0, result3.1);
}
