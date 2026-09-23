fn main() {
    let f1 = f32::NAN;
    let f2 = f32::NAN;
    let result = float_compare(f1, f2);
    println!("{}", result);
}

pub fn float_compare(f1: f32, f2: f32) -> bool {
    /* your code here */
    f1 == f2
}