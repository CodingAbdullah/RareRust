fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    sub_by_index(&mut v);
    println!("{:?}", v);
}

pub fn sub_by_index(v: &mut Vec<i32>) {
    // your code here
    for (i, v) in v.iter_mut().enumerate() {
        *v = *v - (i as i32);
    }
}
