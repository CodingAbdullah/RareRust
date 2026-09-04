use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 2, y: 3 };
    
    let mut hasher = DefaultHasher::new();
    p1.hash(&mut hasher);
    let result = hasher.finish();
    
    // your code here
}
