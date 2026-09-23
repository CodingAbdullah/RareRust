// your code here
pub fn first_to_last<T>(v: Vec<T>) -> Vec<T> {
    let mut new_vec: Vec<T> = v;

    if new_vec.len() == 0 {
        return new_vec;
    }
    else {
        let item = new_vec.remove(0);
        new_vec.push(item);

        return new_vec;
    }
}

fn main() {
    let v = vec![1,2,3,4];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v = vec!["a".to_string(), "b".to_string()];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v = vec![5.3];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_to_last(v);
    println!("{:?}", result);

}