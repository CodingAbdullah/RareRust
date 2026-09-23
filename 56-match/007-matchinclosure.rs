fn main() {
    let v = vec![Some(1), None, None, Some(-1)];
    let result = abs_inside_value(&v);
    println!("{:?}", result); // Output: [Some(1), None, None, Some(1)]
}

pub fn abs_inside_value(v: &[Option<i32>]) -> Vec<Option<i32>> {
    v.iter().map(|&x| {
        match x {
            Some(val)  => {
                if x.unwrap() < 0 {
                    Some(-val)
                }
                else {
                    Some(val)
                }
                // your code here
                // hint: you can return inside a closure
                // by omitting the semicolon
            }
            _ => x,
        }
    }).collect()
}