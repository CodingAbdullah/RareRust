fn main() {
    let a: [u16; 5] = [235, 254, 255, 256, 257];
    let result: Vec<u8> = downcasted(a);
    println!("{:?}", result);
}

pub fn downcasted(v: [u16; 5]) -> Vec<u8> {
    // your code here
    let u8_max = u8::MAX;

    v.into_iter().filter(|&x| { x <= u8_max as u16 }).map(|x| { x as u8 }).collect()
}