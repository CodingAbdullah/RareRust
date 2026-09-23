fn main() {
    let v = vec![1, 2, 3];
    let res = append_zero(v);
    println!("{:?}", res);
   // println!("{:?}", v); // comment this out so the code compiles
}

pub fn append_zero(mut v: Vec<i32>) -> Vec<i32> {
    v.push(0);
    v
}
