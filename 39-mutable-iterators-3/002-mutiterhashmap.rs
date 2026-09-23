use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::from([(1,10), (2, 20), (3, 30)]);
    
    for (_k, v) in &mut hm {
        *v = *v + 1;
       // *k = *k + 1;
    }
    
    println!("{:?}", hm);
}