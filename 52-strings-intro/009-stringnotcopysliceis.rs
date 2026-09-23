fn main() {
    let string_slice: &str = "Hello, world!";
    let actual_string: String = vec!['a', 'b', 'c'].into_iter().collect();

    let copy1 = string_slice;
    println!("{}", string_slice);

    let copy2 = actual_string; // your code here
    //println!("{}", actual_string);

    // use copies to avoid compiler warning
    println!("{} {}", copy1, copy2);
}