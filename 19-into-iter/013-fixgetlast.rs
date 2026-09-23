fn main() {
    let v = vec![1, 2, 3];

    let result = get_last(&v);

    println!("{:?}", result);
}

pub fn get_last(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }
    let last = v.iter().last().unwrap();
    Some(*last)
}