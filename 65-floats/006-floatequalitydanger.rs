fn main() {

    let x = 0.1;
    let y = 0.2;
    let z = 0.3;
    
    println!("{:?}", sum_eq(x, y, z));
}

pub fn sum_eq(x: f64, y: f64, z: f64) -> bool {
    // your code here
    z == x + y
}