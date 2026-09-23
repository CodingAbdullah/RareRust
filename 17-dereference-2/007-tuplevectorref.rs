fn get_num(pair: &(Vec<i32>, i32)) -> (Vec<i32>, i32) {
    (pair.0.clone(), pair.1) // remove the * and .clone() this
}

fn main() {
    let p = (vec![4, 5], 9);
    println!("{:?}", get_num(&p));
}
