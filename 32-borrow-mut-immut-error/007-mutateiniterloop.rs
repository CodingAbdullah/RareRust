use std::collections::HashMap;

fn main() {
    
    let mut a: HashMap<i32, i32> = HashMap::from([(2, 10), (3, 30)]);
    
    let result = remove_even_keys(a);
    println!("{:?}", result);
}

pub fn remove_even_keys(mut a: HashMap<i32, i32>) -> HashMap<i32, i32> {

    let keysvec: Vec<i32> = a.keys().copied().collect();

    for k in 0..keysvec.len() {
        if keysvec[k] % 2 == 0 {
            a.remove(&keysvec[k]);
        }
        
    }
    a
}