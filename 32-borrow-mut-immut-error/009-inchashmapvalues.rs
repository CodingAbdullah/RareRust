use std::collections::HashMap;

fn main() {
    
    let mut hm = HashMap::from([(1,2), (2, 4), (3, 6)]);
    
    inc_all_values(&mut hm);
    println!("{:?}", hm);
}

pub fn inc_all_values(hm: &mut HashMap<i32, i32>) {
    
    let keyvec: Vec<i32> = hm.keys().copied().collect();

    for i in 0..keyvec.len() {
        hm.insert(keyvec[i], *hm.get(&keyvec[i]).unwrap() + 1);
    }
}