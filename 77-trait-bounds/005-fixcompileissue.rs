pub fn first_two<T:Clone>(s: &[T]) -> Option<(T, T)> {
    if s.len() < 2 {
        return None
    }
    
    Some((s[0].clone(), s[1].clone()))
}

fn main() {
    
    let v1 = vec![1, 2, 3];
    let result = first_two(&v1);
    println!("{:?}", result);

    let v2 = [vec![1], vec![2]];
    let result = first_two(&v2);
    println!("{:?}", result);
}