fn main() {
    let v = vec![1, 2, 3];

    let result = v.into_iter().sum();
    println!("{}", foo(result));
}

pub fn foo(x: i32) -> i32 {
    x
}
