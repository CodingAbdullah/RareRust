fn main() {
    let x = 42;
    let result = double(x);
    println!("{}", result);
}

pub fn double(x: i32) -> i32 {
    // your code here
    x*2
}