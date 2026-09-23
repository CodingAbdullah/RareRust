fn main() {
    let a = [(1, 2), (3, 4), (5, 6)];
    let result = sum_tuples(&a);
    println!("{:?}", result);
}

pub fn sum_tuples(a: &[(i32, i32)]) -> Vec<i32> {
    // your code here
    a.iter().map(|&(x, y)| { x+y}).collect()
}
