fn main() {
    let v = vec![1, 2, 10];
    let result = less_than_ten(v);
    println!("{:?}", result); // Output: [true, true, false]
}

pub fn less_than_ten(v: Vec<i32>) -> Vec<bool> {
    v.iter().map(|x| {
        if *x < 10 {
            return true;
        }
        else {
            return false;
        }
    }).collect()
}
