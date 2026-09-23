fn main() {
    // &Vec<i32> to &Vec<&i32>
    let v2: &Vec<i32> = &vec![1,2,3];
    let _vc2: &Vec<&i32> = &v2.iter().collect();
  } 