fn main() {
    let v = vec![Some(10), None, None, Some(20)];
    
    let result = unwrap_all_somes(v);
    println!("{:?}", result);
}

pub fn unwrap_all_somes(v: Vec<Option<i32>>) -> Vec<i32> {
    // your code here
    v.into_iter().filter(|&x| { x != None }).map(|x| { x.unwrap() }).collect()
}