fn main() {
    let v = vec![10,11,12];
    
    let result = mul_by_index(v);
    println!("{:?}", result);
}

pub fn mul_by_index(v: Vec<i32>) -> Option<i32> {
    let mut modified_sum = 0;

    if v.len() == 0 {
        return None;
    }
    else {

        for (i, value) in v.iter().enumerate() {
            modified_sum = modified_sum + (i as i32)*(value);
        }    

        Some(modified_sum)
    }
}