fn main() {
    let b = true;
    let result = zero_one(b);
    println!("{}", result);

    let b = false;
    let result = zero_one(b);
    println!("{}", result);
}

pub fn zero_one(b: bool) -> i32 {
    let n: i32 = i32::from(b);
    n
}

