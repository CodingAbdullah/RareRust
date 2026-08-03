fn main() {
    let x = 3;
    let n = 4;
    let product = prod(x, n);
    println!("{}", product);
}

pub fn prod(x: u32, n: u32) -> u32 {
    let mut p = 0;
    // prefix i with _i to silence the compiler warning
    for i in 0..n {
        p = p + x;
    }
    p
}