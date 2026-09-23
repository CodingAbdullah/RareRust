fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5];

    let result = bigger(v);
    println!("{:?}", result);
}

pub fn bigger(v: Vec<u8>) -> Vec<u16> {
    // your code here
    let mut newvec: Vec<u16> = Vec::new();

    for value in &v {
        let item = u16::from(*value);
        newvec.push(item);
    }

    newvec
}
