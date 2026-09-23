fn main() {
    let a = [1, 2, 3];
    let result = product_is_even(&a);
    println!("{}", result);
}

pub fn product_is_even(sl: &[i32]) -> bool {
    sl.into_iter().product::<i32>() % 2 == 0
}
