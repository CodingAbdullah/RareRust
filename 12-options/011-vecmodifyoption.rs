fn main() {
    let v = vec![1,2,3];
    let result_ok = double_at(&v, 1);
    println!("Double index 1: {:?}", result_ok);

    let result_none = double_at(&v, 5);
     println!("Double index 5: {:?}", result_none);

     let result_empty = double_at(&Vec::<i32>::new(), 0);
     println!("Double index 0 (empty): {:?}", result_empty);
}

// pub fn double_at your code here
pub fn double_at(v: &Vec<i32>, k: i32) -> Option<Vec<i32>> {
    let mut v_clone = v.clone();

    if k < 0 || k >= v_clone.len() as i32 {
        return None;
    }
    else {
        v_clone[k as usize] = v_clone[k as usize]*2;
    }

    Some(v_clone)
}