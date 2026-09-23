pub fn max<T: PartialOrd >(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let result = max('a', 'c');
    println!("{}", result);

    let result = max(true, false);
    println!("{}", result);
    
    let result = max(3.0, 2.0);
    println!("{}", result);
}