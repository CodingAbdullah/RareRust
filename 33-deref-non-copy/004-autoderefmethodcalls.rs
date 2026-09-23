use std::collections::HashSet;

fn main() {
    let mut hs = HashSet::from([1,2,3]);
    
    let mut_ref_hs = &mut hs;
    
    mut_ref_hs.insert(4);
    (mut_ref_hs).insert(5);
    
    println!("{:?}", hs);
}