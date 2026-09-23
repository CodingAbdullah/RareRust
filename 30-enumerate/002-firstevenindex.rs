fn main() {
    let v = vec![1, 3, 4, 5, 6];
    
    let result = first_even_index(v);

    println!("{:?}", result);
}

pub fn first_even_index(v: Vec<i32>) -> Option<usize> {
    // your code here
    for (i, value) in v.iter().enumerate() {
        if *value % 2 == 0 {
            return Some(i);
        }
    }

    None
}