fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    
    let result = add_max_to_all(&v1, v2);
    println!("{:?}", result); // [7,8,9]
}

pub fn add_max_to_all(first: &Vec<i32>, mut second: Vec<i32>) -> Vec<i32> {
    // your code here
    let max_value = first.iter().max();

    if max_value.is_none() {
        return second;
    }
    else {
        for i in 0..second.len() {
            second[i] = second[i] + max_value.unwrap();
        }
    }

    second
} 