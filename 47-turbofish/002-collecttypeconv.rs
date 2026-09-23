use std::collections::HashSet;
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let set = v.into_iter().collect::<HashSet<i32>>();
    println!("{:?}", set);
}
