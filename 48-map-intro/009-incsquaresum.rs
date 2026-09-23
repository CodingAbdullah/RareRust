fn main() {
    let v = vec![1, 2, 3];
    let result = inc_square_sum(v);
    println!("{}", result);
}

pub fn inc_square_sum(v: Vec<i32>) -> i32 {
    v.into_iter().map(|x| {x+1}).map(|y| {y*y}).sum()
}
