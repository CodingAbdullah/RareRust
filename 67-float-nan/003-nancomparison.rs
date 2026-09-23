fn main() {
    let result = compare_nan(1.0);
    println!("{:?}", result);
}

pub fn compare_nan(x: f32) -> (bool, bool, bool, bool) {
    (f32::NAN > x, f32::NAN < x, f32::NAN >= x, f32::NAN <= x)
}

