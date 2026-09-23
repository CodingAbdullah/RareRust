fn main() {
    let a = [((1, 2), (3, 4)), ((5, 6), (7, 8))];
    let result = sum_nested_tuples(a);
    println!("{:?}", result); // Output: [(3, 7), (11, 15)]
}

pub fn sum_nested_tuples(arr: [((i32, i32), (i32, i32)); 2]) -> Vec<(i32, i32)> {
    // your code here
    arr.into_iter().map(| ((a, b), (c, d)) | {
        (a + b, c + d)
    }).collect()
}
