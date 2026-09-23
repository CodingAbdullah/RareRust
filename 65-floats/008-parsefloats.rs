fn main() {
    let v = vec!["1.0", "hello", "0.1"];
    let result = convert_vec(v);
    println!("{:?}", result);
}

pub fn convert_vec(v: Vec<&str>) -> Vec<f32> {
    
    // your code here
    v.iter()
    .map(|x| { x.parse::<f32>() })
    .filter(|x| { x.is_ok() })
    .map(|x| x.unwrap() )
    .collect()
}