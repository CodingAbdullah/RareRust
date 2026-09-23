// your code here
pub fn first_two<T>(v: Vec<T>) -> Option<Vec<T>> {
    if v.len() < 2 {
        return None;
    }
    else {
        return Some(v.into_iter().take(2).collect::<Vec<T>>());
    }
}

fn main() {
    let v = vec![1,2,3,4];
    let result = first_two(v);
    println!("{:?}", result);
    
    let v = vec![1.0,2.0,3.0,4.0];
    let result = first_two(v);
    println!("{:?}", result);
}