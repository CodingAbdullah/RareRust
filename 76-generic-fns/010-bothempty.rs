// your code here
pub fn both_empty<T, U>(v1: Vec<T>, v2: Vec<U>) -> bool {
    if v1.len() == 0 && v2.len() == 0 {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let a: Vec<String> = Vec::new();
    let b: Vec<char> = Vec::new();
    let result = both_empty(a, b);
    println!("{:?}", result);
}