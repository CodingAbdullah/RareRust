#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct S {
    pub f: u32,
}


fn main() {
    
    let s1 = S { f: 3 };
    let s2 = S { f: 1 };
    let s3 = S { f: 2 };
    
    let mut a = [s1, s2, s3];
    a.sort();
    
    println!("{:?}", a);
}