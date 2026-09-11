pub fn product_of_coordinates(coord: (i32, i32, i32)) -> i32 {
    // your code here
    coord.0 * coord.1 * coord.2
}

fn main() {
    let coord = (12, 6, 5);
    println!("Product of Cordinates: {}", product_of_coordinates(coord));
}
