use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Mono {
    a: i32,
}

fn main() {
    let hs = HashSet::from([
        Mono { a: 1 },
        Mono { a: 2 },
    ]);
    
    println!("{:?}", hs);
}