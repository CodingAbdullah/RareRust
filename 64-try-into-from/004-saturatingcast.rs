fn main() {
    let v = vec![10, 70000, 65535, 0];
    let out = saturating_downcast(v);
    println!("{:?}", out); // [10, 65535, 65535, 0]
}

pub fn saturating_downcast(v: Vec<u32>) -> Vec<u16> {
    // your code here

    v.iter().map(|&x| {
        let result: Result<u16, _> = u32::try_into(x);
        match result {
            Ok(x) => x,
            Err(_) => u16::MAX,
        }
    }).collect()
}
