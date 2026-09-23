fn main() {
    let a = vec![1, 3, 3, 4, 5, 6, 7, 8, 9];
    let result = odds(&a);
    println!("{}", result);
}

pub fn odds(a: &[i32]) -> usize {
    // your code here
    a.iter().filter(| &&x| { x % 2 != 0 }).count()
}