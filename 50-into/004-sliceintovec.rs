fn main() {
    let a = [1, 2, 3];
    let result = rest_to_vec(&a);
    println!("{:?}", result);
}

pub fn rest_to_vec(s: &[i32]) -> Vec<i32> {
    // your code here
    if s.len() == 0 {
        return vec![];
    }
    else {
        let newvec: Vec<i32> = s[1..].into();
        newvec
    }
}