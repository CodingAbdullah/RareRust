fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let result = v.into_iter().map(increment).collect::<Vec<i32>>();
    println!("{:?}", result);
}

pub fn increment(x: i32) -> i32 {
    x + 1
}
