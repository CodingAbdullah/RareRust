fn main() {
    // &Vec<&i32> to &Vec<i32>
    let v12: &Vec<&i32> = &vec![&1,&2,&3];
    let _v12: &Vec<i32> = &v12.clone().iter().copied().copied().collect();
}