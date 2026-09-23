// your code here
pub fn find_last<T: PartialEq>(v: Vec<T>, s: T) -> Option<usize> {
    let mut exists: bool = false;
    let mut latest_idx: usize = 0;

    for (i, item) in v.into_iter().enumerate() {
        if item == s {
            exists = true;
            latest_idx = i;
        }
    }
    
    if exists {
        return Some(latest_idx);
    }
    else {
        return None;
    }
}

fn main() {
    let v = vec![ "world".to_string(), "hello".to_string(), "world".to_string(), "RareCode".to_string()];
    let result = find_last(v, "world".to_string());
    println!("{:?}", result); // Some(2)
}