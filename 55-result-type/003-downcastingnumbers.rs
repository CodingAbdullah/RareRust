fn main() {
    let nums: Vec<u16> = vec![1, 2, 999];
    let result = downcast(&nums);
    println!("{:?}", result);
}

pub fn downcast(slice: &[u16]) -> Vec<u8> {
    // your code here
    slice
    .iter()
    .map(|x| { u8::try_from(*x)})
    .filter(|&y| { y.is_ok() })
    .map(|z| { z.unwrap() }).collect()
}
