// your code here
pub fn len_compare<T, U>(v1: Vec<T>, v2: Vec<U>) -> bool {
    if v1.len() > v2.len() {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let result = len_compare([1,2,3].to_vec(), [true, false].to_vec());
    println!("{:?}", result);
}