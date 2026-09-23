fn main() {
    let v = vec![1, 2, 3];
    let result = get_last(v);
    println!("{}", result);
}

pub fn get_last(v: Vec<i32>) -> i32 {
    // your code here
    let value = v.into_iter().last();

    if value == None {
        0
    }
    else {
        value.unwrap()
    }
}
