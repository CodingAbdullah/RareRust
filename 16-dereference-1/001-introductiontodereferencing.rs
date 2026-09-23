
fn main() {
    let vector = vec![1, 2, 3];
    let result = sum_doubles(&vector);
    println!("{}", result);
}

pub fn sum_doubles(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += double(*e); // dereference e
    }
    s
}

pub fn double(e: i32) -> i32 {
    e * 2
}
