// your code here
pub fn find_last<T: PartialEq>(v: &[T], s: T) -> Option<usize> {
    if v.len() == 0 {
        return None;
    }
    else {
        let mut doesexist: bool = false;
        let mut last_element: usize = 0;

        for (i, item) in v.into_iter().enumerate() {
            if *item == s {
                last_element = i;
                doesexist = true;
            }
        }

        if doesexist {
            return Some(last_element);
        }
        else {
            return None;
        }
    }
}

fn main() {
    let v = vec![ "world".to_string(), "hello".to_string(), "world".to_string(), "RareCode".to_string()];
    let result = find_last(&v, "world".to_string());
    println!("{:?}", result);
    
    let v = [1,2,3,2];
    let result = find_last(&v, 3);
    println!("{:?}", result);
}