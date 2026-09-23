fn main() {
    let v = vec![1, 2, 3];
    let result = sum(&v);
    println!("{}", result);
}

pub fn sum(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += *e; // Rust auto-dereferences here. Change to *e. Both work.
    }
    s
}
