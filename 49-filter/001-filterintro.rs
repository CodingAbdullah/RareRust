fn main() {
    let v = vec![1, 2, 3, 4];
    
    let even_v = v.into_iter().filter(|x| { x % 2 ==0 })
    .collect::<Vec<i32>>();
    println!("{:?}", even_v); // [2, 4]
}