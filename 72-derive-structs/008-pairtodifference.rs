use std::collections::HashMap;

// your code here
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pair {
   pub a: i32,
   pub b: i32,
}

fn main() {
    let v = vec![
        Pair { a: 1, b: 2 },
        Pair { a: 1, b: 2 },
        Pair { a: 3, b: 3 },
    ];
    
    let result = to_difference(&v);
    println!("{:?}", result);
}

pub fn to_difference(v: &[Pair]) -> HashMap<Pair, i32> {
    v.iter().map(|&x| {
        (x, x.a - x.b)
    }).collect()
}