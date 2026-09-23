use std::collections::HashMap;

fn main() {
    let v = vec![&(1,10),&(2,20),&(3,30)];
    
    let result = convert(v);
    
    println!("{:?}", result);
}

pub fn convert(v: Vec<&(i32, i32)>) -> HashMap<i32, i32> {
	// your code here
    v.into_iter().copied().collect()
}
