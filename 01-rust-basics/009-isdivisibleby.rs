fn main() {
    let x = 42;
    let result = is_divisible_by(x, 7);
    println!("{}", result);
}

pub fn is_divisible_by(x: i32, y: i32) -> bool {
    x % y == 0
}

/*

x % y == 0 returns a boolean. The function could also be written as

pub fn is_divisible_by(x: i32, y: i32) -> bool {
    if x % y == 0 {
        return true;
    }

    false
}

But `x % y == 0` is a lot cleaner

*/