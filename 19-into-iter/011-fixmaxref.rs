fn main() {
    let v = vec![1, 2, 3];

    let result = get_max(&v);

    println!("{:?}", result);
}

pub fn get_max(v: &Vec<i32>) -> Option<i32> {
    // your code here
    if v.len() == 0 {
        return None;
    }
    else {
        let maxvalue = v.iter().max().unwrap();
        Some(*maxvalue)
        
    }
}
