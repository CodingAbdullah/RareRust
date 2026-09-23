fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    add_to_index(&mut v);

    println!("{:?}", v);
}

pub fn add_to_index(v: &mut Vec<i32>) {
    for (i, e) in v.iter_mut().enumerate() {
        *e = *e + (i as i32);
    }
}