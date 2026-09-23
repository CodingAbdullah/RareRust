fn main() {
    let v = vec![1, 2, 3];
    let result = double_product(&v);
    println!("{}", result);
}

pub fn double_product(v: &[i32]) -> i32 {
    v.iter().product::<i32>() * 2
}