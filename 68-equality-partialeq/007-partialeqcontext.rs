fn main() {
    let v1 = vec![1.0, 2.0, f64::NAN, 3.0];
    let v2 = vec![1.0, 2.0, f64::NAN, 3.0];
    
    println!("Direct comparison: {}", v1 == v2);
    println!("Smart comparison: {}", float_vecs_match(&v1, &v2));
}

pub fn float_vecs_match(v1: &Vec<f64>, v2: &Vec<f64>) -> bool {
    let pct_delta = 0.01;

    if v1.len() != v2.len() {
        return false;
    }
    else {
        for i in 0..v1.len() {
            let v1_value_abs = f64::abs(v1[i]);
            let v2_value_abs = f64::abs(v2[i]);

            if v1[i].is_nan() && v2[i].is_nan() {
                continue;      
            }
            else if v1[i] >= v2[i] {
                if ((f64::abs(v1_value_abs - v2_value_abs))/(v2_value_abs)) <= pct_delta {
                    continue;
                }
                else {
                    return false;
                }
            }
            else if v1[i] < v2[i] {
                if ((f64::abs(v1_value_abs - v2_value_abs))/(v1_value_abs)) <= pct_delta {
                    continue;
                }
                else {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        return true;
    }
}