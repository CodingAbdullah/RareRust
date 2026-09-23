fn main() {
    
    let x = 9.9;
    let y = 10.0;
    let pct_delta = 0.012;
    let result = approx_equal(x, y, pct_delta);
    println!("{}", result);
}

pub fn approx_equal(x: f32, y: f32, pct_delta: f32) -> bool {
    // your code here
    let abs_x = f32::abs(x);
    let abs_y = f32::abs(y);

    if abs_x <= abs_y {
        if (f32::abs(abs_x - abs_y))/(abs_x) <= pct_delta {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        if (f32::abs(abs_x - abs_y))/(abs_y) <= pct_delta {
            return true;
        }
        else {
            return false;
        }
    }
}