pub fn increment<T>(x: T) -> i32
where
    T: std::borrow::Borrow<i32>,
{
    *x.borrow() + 1
}

fn main() {
    let x = 1;
    println!("{}", /* call increment with the value x */increment(x)); // 2
    println!("{}", /* call increment with a reference to x */increment(&x)); // 2
}
