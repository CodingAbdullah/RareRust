// accept a reference instead
pub fn accept_tuple(_t: &(i32, bool, Vec<i32>)) {

}

fn main() {
    let data = (99, true, vec![1, 2, 3]);
    accept_tuple(&data); // pass a reference instead
    println!("{:?}", data);
}