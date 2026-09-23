// declare the struct here
#[derive(Debug)]
pub struct Pair {
    pub a: i32,
    pub b: i32
}

fn main() {
    let mut v = vec![Pair { a: 3, b: 4 }, Pair { a: 10, b: 9 }, Pair { a: 14, b: 14 }];
    
    to_zero(&mut v);
    println!("{:?}", v);
}

pub fn to_zero(v: &mut Vec<Pair>) {
    // your code here
    for i in 0..v.len() {
        if v[i].a < v[i].b {
            v[i].a = 0;
        }
        else if v[i].a > v[i].b {
            v[i].b = 0;
        }
    }
}