use std::collections::HashSet;

fn main() {
    
    let v = vec![&&1, &&2, &&3];
    let mut s: HashSet<i32> = HashSet::new();
    
    assign(&mut s, v);
    
    println!("{:?}", s);
}

pub fn assign(s: &mut HashSet<i32>, v: Vec<&&i32>) {
    // your code here
    *s = v.iter().copied().copied().copied().collect(); 
}