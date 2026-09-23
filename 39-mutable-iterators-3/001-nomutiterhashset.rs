use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
   // for e in set.iter_mut() {
   //     *e = *e * *e;
   // }
    
    println!("{:?}", set);
}