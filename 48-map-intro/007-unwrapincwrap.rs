fn main() {
    let v = vec![Some(5), Some(15), None];
    let result = inc_option(v);
    println!("{:?}", result); // Output: [Some(6), Some(16), None]
}

pub fn inc_option(v: Vec<Option<i32>>) -> Vec<Option<i32>> {
    v.into_iter()
        .map(|x| {
            // your code here
            if x.is_some() {
                let value = x.unwrap() + 1;
                return Some(value);
            }
            None
        }).collect()
}
