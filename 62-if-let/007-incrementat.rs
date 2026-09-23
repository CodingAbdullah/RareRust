fn main() {
    let mut v = vec![1, 2, 3];
    
    inc_at(&mut v, 2);
    println!("{:?}", v);
}

pub fn inc_at(v: &mut Vec<i32>, idx: usize) -> Option<i32> {
    if let Some(value) = v.get_mut(idx) {
        *value = *value + 1;
        return Some(*value);
    }
    else {
        return None;
    }
    // your code here
}