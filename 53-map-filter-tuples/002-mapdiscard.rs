fn main() {
    let a = [(1, 2, 3), (4, 5, 6), (7, 8, 9)];
    let result = vec_from_middle(a);
    println!("{:?}", result);
}

pub fn vec_from_middle(a: [(i32, i32, i32); 3]) -> Vec<i32> {
    // your code here
    a.into_iter().map(|(_, b, _)| { b }).collect()
}
