fn main() {
    let values1 = [1.0, 2.0, 3.0];
    let values2 = [1.0, f32::NAN, 3.0];
    let values3 = [f32::NAN, f32::INFINITY];

    println!("{:?}", average_ignore_nan(&values1));
    println!("{:?}", average_ignore_nan(&values2));
    println!("{:?}", average_ignore_nan(&values3));
}

pub fn average_ignore_nan(values: &[f32]) -> Option<f32> {
    // your code here
    let mut sum = 0.0;
    let mut val_count = 0.0;

    for val in values {
        if val.is_nan() || val.is_infinite() {
            continue;
        }
        else {
            sum += val;
            val_count += 1.0;
        }
    }
    
    if val_count == 0.0 {
        return None;
    }
    else {
        return Some(sum/(val_count));
    }

}

