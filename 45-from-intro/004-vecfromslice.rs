fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    let result: Vec<i32> = Vec::from(slice);
    println!("{:?}", result);
}