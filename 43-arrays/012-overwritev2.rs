fn main() {
    let mut a = [1, 2, 5, 4, 3];
    let b = [&&4, &&5, &&6, &&7, &&8];
    
    overwrite_v2(&mut a, b);
    println!("{:?}", a); // [4,5,6,7,8]
}

pub fn overwrite_v2(a: &mut [i32; 5], b: [&&i32; 5]) {
    // your code here
    for (i, e) in a.iter_mut().enumerate() {
        *e = **b[i];
    }
}