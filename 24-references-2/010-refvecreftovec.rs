fn main() {
    // &Vec<&i32> to Vec<i32>
let v: &Vec<&i32> = &vec![&1,&2,&3];
let _v: Vec<i32> = v.iter().copied().copied().collect();
}