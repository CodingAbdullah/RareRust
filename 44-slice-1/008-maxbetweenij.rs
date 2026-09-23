fn main() {
    let v: Vec<i32> = vec![4,1,3,2];

    let result = max_between(v.clone(), 1, 3);
    println!("Expect: Some(3) Got: {:?}", result);
    
    let result = max_between(v.clone(), 0, 3);
    println!("Expect: Some(4) Got: {:?}", result);
    
    let result = max_between(v.clone(), 1, 1);
    println!("Expect: None Got: {:?}", result);

    let result = max_between(v.clone(), 1, 4);
    println!("Expect: Some(3) Got: {:?}", result);
    
    let result = max_between(v.clone(), 3, 1);
    println!("Expect: None Got: {:?}", result);
}

pub fn max_between(v: Vec<i32>, i: usize, j: usize) -> Option<i32> {
    // your code here
    if i >= j {
        return None; 
    }
    else if i > v.len() || j > v.len() {
        return None;
    }
    else {
        let max_value = v[i..j].iter().max().copied().unwrap();
        return Some(max_value);
    }
}