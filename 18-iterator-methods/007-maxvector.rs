fn main() {
    let vv = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 7]];

    let result = max_of_each(vv);
    println!("{:?}", result);
}

pub fn max_of_each(vv: Vec<Vec<i32>>) -> Vec<Option<i32>> {
    // your code here
    let mut vecoption: Vec<Option<i32>> = Vec::new();

    for i in 0..vv.len() {
        let vv_clone = vv[i].clone();
        vecoption.push(vv_clone.into_iter().max());
    }

    vecoption
}
