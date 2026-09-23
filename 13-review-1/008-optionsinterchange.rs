pub fn from_index(v: Vec<i32>/* add type */, start: i32/* add type */) -> Option<Vec<i32>> {
    let mut result: Vec<i32> = Vec::new();
    // Your logic here
    if start < 0 || start as usize >= v.len() {
        return None;
    }
    else {
        for i in start..v.len() as i32 {
            result.push(v[i as usize]);
        }
    }

    Some(result)
}

fn main() {
    let vector = vec![-2, 1,2,3,4,5,6,7,8];
    let result = from_index(vector, 7);

    println!("{:?}", result);

    let vector = vec![7,8];
    let result = from_index(vector, 2);

    println!("{:?}", result); // should be None
}