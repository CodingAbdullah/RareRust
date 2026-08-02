fn main() {
    let x = 42;
    let y = 2;
    let op = 0;
    let result = calculator(x, y, op);
    println!("{}", result);
}

pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    // your code here
    if op == 0 {
        x + y
    }
    else if op == 1 {
        x - y
    }
    else if op == 2 {
        x * y
    }
    else if op == 3 {
        x / y
    }
    else {
        x % y
    }
} 