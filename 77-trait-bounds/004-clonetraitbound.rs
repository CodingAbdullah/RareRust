pub fn clone_elements<T:Clone>(s: &Vec<T>) -> Vec<T> {
    s.into_iter().map(|x| x.clone()).collect()
}

fn main() {
    let v = vec!["hello", "world"];
    let result = clone_elements(&v);

    println!("{:?}", result);
    
    let v = vec![1, 2];
    let result = clone_elements(&v);
    println!("{:?}", result);
}