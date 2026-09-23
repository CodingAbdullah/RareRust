fn main() {
    let v = vec![1,2,3,4];
    let result = last_n(&v, 2);
    println!("{:?}", result);
}

pub fn last_n(sl: &[i32], n: usize) -> Vec<i32> {
    if n > sl.len() {
        let mut newvec: Vec<i32> = Vec::new();
        for item in sl {
            newvec.push(*item);
        }
        return newvec;
    }
    else {
        let lastelements = sl.len() - n;

        let newvec: Vec<i32> = sl[lastelements..].iter().copied().collect();
        newvec
    }
    // your code here
}