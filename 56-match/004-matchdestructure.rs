fn main() {
    let t = (0, 1);
    let result = increment_if_not_zero(t);
    println!("{:?}", result);
}

pub fn increment_if_not_zero(t: (u32, u32)) -> (u32, u32) {
    match t {
        (0, 0) => (0, 0),
        (0, y) => (0, y + 1),
        (x, 0) => (x + 1, 0),
        (x, y) => (x + 1, y + 1)
    }
}