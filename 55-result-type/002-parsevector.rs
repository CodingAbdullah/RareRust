fn main() {
    let v = ["1", "2", "h", "3"];
    let result = parse_or_remove(&v);
    println!("{:?}", result); // Should print: [1, 2, 3]
}

pub fn parse_or_remove(v: &[&str]) -> Vec<i16> {
    // your code here

    v.iter()
    .map(|x| {
        x.parse::<i16>()
    })
    .filter(|y| { y.is_ok() })
    .map(|z| { z.unwrap() })
    .collect()
}