fn main() {
    let v = vec![Some(10), None, Some(42), None, None];
    
    let result = remove_nones(v);
    println!("{:?}", result);
}

pub fn remove_nones(v: Vec<Option<i32>>) -> Vec<Option<i32>> {
    // your code here
    v.into_iter().filter(|&x| { x != None }).collect()
}