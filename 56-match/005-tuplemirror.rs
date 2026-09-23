fn main() {
    let t = (0, 0, 2);
    let result = tuple_mirror(t);
    println!("{:?}", result);
}

pub fn tuple_mirror(t: (i32, i32, i32)) -> (i32, i32, i32) {
    // your code here
    match t {
        (a, 0, c) => (c, 0, a),
        (a, b, c) => (a, b, c)
    }
}
