fn main() {
    println!("Looping with implicit into_iter():");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("Looping with explicit into_iter():");
    // TODO convert the range to an iterator with (0..10).into_iter()
    for i in (0..10).into_iter() {
        println!("{}", i);
    }
}
