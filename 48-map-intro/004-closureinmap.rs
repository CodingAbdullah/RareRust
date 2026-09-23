fn main() {
    let v = [1, 2, 3];
    let sum_of_squares: i32 = v.into_iter().map(|x| x*x).sum();
    println!("{}", sum_of_squares);
}
