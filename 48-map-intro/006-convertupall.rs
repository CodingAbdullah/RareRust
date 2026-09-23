fn main() {
    let v = vec![1, 2, 3];
    let result = convert_up(v);
    println!("{:?}", result);
}

pub fn convert_up(v: Vec<i32>) -> Vec<i128> {
    v.iter().map(|x| { i128::from(*x) }).collect()

}
