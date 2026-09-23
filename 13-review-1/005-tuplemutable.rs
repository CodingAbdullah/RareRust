pub fn increment_second(t: &(i32, i32)) -> (i32, i32) {
    // your code here
    let mut mut_t = t.clone();
    mut_t.1 = mut_t.1 + 1;

    return mut_t;
}

fn main() {
    let my_tuple = &(10, 20);
    let updated_tuple = increment_second(my_tuple);
    println!("Original: {:?}, Updated: {:?}", my_tuple, updated_tuple); // Original: (10, 20), Updated: (10, 21)

    let another_tuple = &(0, -5);
    let updated_another_tuple = increment_second(another_tuple);
    println!(
        "Original: {:?}, Updated: {:?}",
        another_tuple, updated_another_tuple
    ); // Original: (0, -5), Updated: (0, -4)
}