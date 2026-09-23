fn main() {
    let x = 10;
    let y = 5;
    let x_ref = &x;
    let y_ref = &y;
    let sum = *x_ref + *y_ref; // Rust auto-dereferences
    println!("Sum: {}", sum);
}
