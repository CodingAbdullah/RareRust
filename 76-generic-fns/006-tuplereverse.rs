// your code here
pub fn rtup<T>(tuple: (T, T)) -> (T, T) {
    let new_tuple: (T,T) = (tuple.1, tuple.0);
    new_tuple

}

fn main() {
    let t = (1, 4);
    let result = rtup(t);
    println!("{:?}", result);
    
    let t = (1.0, 4.0);
    let result = rtup(t);
    println!("{:?}", result);
}