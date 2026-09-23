use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let hm = HashMap::from([(1,10), (2, 20), (3, 30)]);
    
    let mut s = HashSet::new();
    
    assign_keys(&mut s, hm);
    
    println!("{:?}", s);
}

pub fn assign_keys(s: &mut HashSet<i32>, hm: HashMap<i32, i32>) {
    *s = hm.keys().copied().collect();
}