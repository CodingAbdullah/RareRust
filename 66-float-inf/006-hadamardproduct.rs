fn main() {
    
    let a1 = [1.0, 2.0, 3.0];
    let a2 = [1.0, 2.0, f32::INFINITY];
    
    let result = element_wise_mul(&a1, &a2);
    println!("{:?}", result); // [Some(1.0), Some(4.0), None]
}

pub fn element_wise_mul(a: &[f32], b: &[f32]) -> Vec<Option<f32>> {
    // your code here
    let mut new_vec: Vec<Option<f32>> = Vec::new();

    for i in 0..a.len() {
        if a[i].is_infinite() || b[i].is_infinite() {
            new_vec.push(None);
        }
        else {
            let product = a[i]*b[i];

            if product.is_infinite() {
                new_vec.push(None);
            }
            else {
                new_vec.push(Some(product));
            }
        }
    }

    new_vec
}