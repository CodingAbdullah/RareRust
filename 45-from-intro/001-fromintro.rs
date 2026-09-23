use std::collections::HashSet;
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let s = HashSet::from(arr);
    println!("{:?}", s);
}
