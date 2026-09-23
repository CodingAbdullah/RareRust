fn main() {
    let v: Vec<i32> = vec![1,2,3];
    
    // convert Vec<i32> into Vec<&i32>
    let v_ref: Vec<&i32> = v.iter().collect();
    
    // convert Vec<&i32> into Vec<i32>
    let _w: Vec<i32> = v_ref.into_iter().copied().collect();
}