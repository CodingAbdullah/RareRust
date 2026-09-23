fn main() {
    let mut v: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
    let w: Vec<Vec<i32>> = vec![vec![1], vec![1, 2], vec![7]];

    assign(&mut v, w);

    println!("{:?}", v);
}

pub fn assign(v: &mut Vec<Vec<i32>>, w: Vec<Vec<i32>>) {
    // your code here
    for (i, e) in w.iter().enumerate() {
        v[i] = (*e).clone();
    }
}
