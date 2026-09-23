// your code here
pub fn pair_lengths<T, U>(v1: Vec<T>, v2: Vec<U>) -> (usize, usize) {
    (v1.len(), v2.len())
}
fn main() {
    let a = vec![1, 2, 3];
    let b = vec!['x', 'y'];
    println!("{:?}", pair_lengths(a, b)); // (3, 2)
}