fn main() {
    let v: Vec<&i32> = vec![&1,&2,&3];
    let _vc: &Vec<i32> = &v.iter().copied().copied().collect();
}