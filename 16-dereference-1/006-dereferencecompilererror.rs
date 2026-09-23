fn main() {
    let start = &0;
    let end = &3;

    for i in *start..*end {
        println!("{}", i);
    }
}
