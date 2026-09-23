fn main() {
    let x = 0.0 / 0.0;
    println!("{}", check_nan(x));
}

pub fn check_nan(x: f32) -> bool {
    // your code here
    if x.is_nan() {
        return true;
    }
    else {
        return false;
    }
}

