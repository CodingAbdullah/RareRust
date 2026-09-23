fn main() {
    
    let mut v: Vec<i32> = vec![];
    
    let r = &v;

    println!("{:?}", r);

    v = vec![1,2,3];
    println!("{:?}", &v);
}