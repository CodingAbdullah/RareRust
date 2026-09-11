fn main() {
    // make t mut
let mut t = (vec![1, 2, 3], 10);
t.0.push(4);  // We try to push a value into a vector in an immutable tuple
println!("{:?}", t);
}