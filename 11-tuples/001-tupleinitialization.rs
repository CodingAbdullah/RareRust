fn main() {
    let x: (i32, bool) = (2, true);
    let y: (u32, i32, bool) = (2, -5, false);
    let z: (Vec<i32>, i32, bool) = (vec![-5, 2, 3], 8, true);
    // your code here
    let p: (i32, i32) = (2, 3);
    let unit: () = (); // empty tuple

    println!("{:?}, {:?}, {:?}, {:?}, {:?}", x, y, z, p, unit);
}
