pub fn first_two<T: Copy>(v: Vec<T>) -> Option<(T, T)> {
    if v.len() < 2 {
        return None;
    }

    Some((v[0], v[1]))
}


fn main() {
    let v = vec![1,2,3,4];
    let result = first_two(v);
    println!("{:?}", result);

    let v = vec![5.3];
    let result = first_two(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_two(v);
    println!("{:?}", result);

}