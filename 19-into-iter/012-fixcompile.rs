fn main() {
    let v = vec![1, 2, 3];

    let result = get_second(&v);

    println!("{:?}", result);
}

pub fn get_second(v: &Vec<i32>) -> Option<i32> {
    if v.len() < 2 {
        return None;
    }
    let scnd = v.iter().nth(1).unwrap();
    Some(*scnd)
}
