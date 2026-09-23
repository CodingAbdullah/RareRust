fn main() {
    let max_f64 = f64::MAX;
    let overflow_to_inf = max_f64 * 2.0;
    println!("{}", overflow_to_inf);
}