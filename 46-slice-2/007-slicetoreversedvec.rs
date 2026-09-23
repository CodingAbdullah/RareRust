fn main() {
    let v = vec![1,2,3];
    let result = reversed(&v[..]);
    println!("{:?}", result);
}

pub fn reversed(v: &[i32]) -> Vec<i32> {
    // do it in one line
    v.into_iter().rev().copied().collect()
}