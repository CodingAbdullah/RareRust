fn main() {
    let a = [1, 0, 4];
    let result = add_to_index(&a);
    println!("{:?}", result);
}

pub fn add_to_index(sl: &[i32]) -> Vec<i32> {
    // your code here
    sl.into_iter().enumerate().map(|(x,y)| { (x as i32) + y }).collect()
}
