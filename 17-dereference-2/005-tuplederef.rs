fn main() {
    let p = (2, 5);
    println!("{:?}", swap_and_double(&p));
}

pub fn swap_and_double(pair: &(i32, i32)) -> (i32, i32) {
    let (a, b) = *pair;
    (b * 2, a * 2) // Swap and double
}
