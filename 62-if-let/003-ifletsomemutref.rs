use std::collections::HashMap;
fn main() {
    
    let mut m = HashMap::from([(1, 100), (2, 200), (3, 300)]);
    
    let k = 3;
    
    inc_if_present(&mut m, &k);
    
    println!("{:?}", m);
}

pub fn inc_if_present(m: &mut HashMap<i32, i32>, k: &i32) {

    if let Some(value) = m.get_mut(k) {
        *value = *value + 1;
    }
    
}