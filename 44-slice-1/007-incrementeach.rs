fn main() {
    let mut v: Vec<i32> = vec![1, 1, 3, 3];

    increment_slice(&mut v[0..2]);

    println!("{:?}", v);
}

pub fn increment_slice(slc: &mut [i32]) {
    // your code here
    for item in slc.iter_mut() {
        *item = *item + 1;
    }
}
