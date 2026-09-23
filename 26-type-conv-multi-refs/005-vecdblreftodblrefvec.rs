fn main() {
    let v = vec![&&1,&&2,&&3];
    
    let _w: &&Vec<i32> = &&(v.into_iter().copied().copied().collect());
}