fn main() {
    let mut a = vec![1, 2, 3, 4];
    let b = vec![1, 1, 1, 1];

    elementwise_add(&mut a, b);
    println!("{:?}", a); // [2,3,4,5];
}

pub fn elementwise_add(a: &mut Vec<i32>, b: Vec<i32>) {
    // your code here

    for (i, e) in b.iter().enumerate() {
        a[i] = *e + a[i];
    }
}
