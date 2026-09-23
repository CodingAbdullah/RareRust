fn main() {
    let v = vec![1, 2, 3];

    let result = sum_and_product(v);
    println!("{:?}", result);
}

pub fn sum_and_product(v: Vec<i32>) -> (i32, i32) {
    // your code here
    ((&v).into_iter().sum(), (&v).into_iter().product())
}
