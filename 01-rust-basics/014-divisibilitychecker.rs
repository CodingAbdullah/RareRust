fn main() {
    let x = 42;
    let result = divisible_by(x);
    println!("{}", result);
}

pub fn divisible_by(x: i32) -> i32 {
    if x % 2 == 0 {
        return 2;
    }
    else if x % 3 == 0 {
        return 3;
    }
    else if x % 5 == 0 {
        return 5;
    }
    else {
        return 0;
    }
} 