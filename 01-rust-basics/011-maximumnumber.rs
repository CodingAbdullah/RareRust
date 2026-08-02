fn main() {
    let x = 42;
    let y = 43;
    let result = max(x, y);
    println!("{}", result);
}

// your code here
pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else if b > a {
        b
    }
    else {
        a
    }
}