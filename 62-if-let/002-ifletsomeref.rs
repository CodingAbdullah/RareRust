fn main() {
    let op = Some(20);
    let result = twice_or_zero(&op);
    println!("{}", result);
}

pub fn twice_or_zero(op: &Option<i32>) -> i64 {
    if let Some(v) = *op {
        return i64::from(v) * 2;
    }
    0
}