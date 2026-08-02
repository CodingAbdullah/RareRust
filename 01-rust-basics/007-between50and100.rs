fn main() {
    let x = 42;
    let result = between_50_and_100(x);
    println!("{}", result);
}

pub fn between_50_and_100(x: i32) -> bool {
    if x < 100 && x > 50 {
        return true;   
    }
    false
}