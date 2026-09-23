fn main() {
    let numbers = vec![10, 20, 30];
    let index_ref = &1;
    println!("{}", numbers[*index_ref]); // run first to see the bug, then explicitly dereference
}
