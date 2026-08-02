fn main() {
    let x = 42;
    let result = floor100(x);
    println!("{}", result);
}

pub fn floor100(x: i32) -> i32 {
    // your code here
    if x < 100 {
        return 100;
    }
    else {
        x
    }
}