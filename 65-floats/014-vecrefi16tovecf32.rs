fn main() {
    let v = Vec::from([&16, &17, &18, &19, &20]);
    
    let result = convert(&v);
    println!("{:?}", result); // [16.0, 17.0, 18.0, 19.0, 20.0]
}

pub fn convert(v: &Vec<&i16>) -> Vec<f32> {
    // your code here
    (*v)
    .clone()
    .into_iter()
    .map(|x| *x)
    .map(|y| { y as f32 })
    .collect()
}