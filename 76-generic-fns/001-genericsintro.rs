pub fn is_even_length<T>(v: Vec<T>) -> bool {
    v.len() % 2 == 0
}


fn main() {
    let v = vec![1,2,3];
    let result = is_even_length(v);
    println!("{:?}", result);
}