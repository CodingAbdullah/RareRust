fn main() {
    let ref_v = &vec![1,2,3];
    
    let _owned_vec: Vec<i32> = ref_v.clone();
}