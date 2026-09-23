fn main() {
    let values = vec![1.0, f32::NAN, f32::INFINITY, -2.5, f32::NEG_INFINITY, 3.3];
    let result = filter_finite(values);
    println!("{:?}", result); // should print [1.0, -2.5, 3.3]
}

pub fn filter_finite(v: Vec<f32>) -> Vec<f32> {
    // your code here
    v.into_iter()
    .filter(|x| {
        if x.is_nan() || x.is_infinite() {
            return false;
        }
        else {
            return true;
        }
    })
    .collect()
}

