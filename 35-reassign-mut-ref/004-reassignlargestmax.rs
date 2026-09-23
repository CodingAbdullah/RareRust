fn main() {
    let mut v: Vec<i32> = vec![];
    
    let a = vec![1,2,3];
    let b = vec![1,2,3,4];
    
    assign_to_vector_with_greatest_max(&mut v, a, b);
    println!("{:?}", v);
}

pub fn assign_to_vector_with_greatest_max(v: &mut Vec<i32>, a: Vec<i32>, b: Vec<i32>) {
    // your code here
    let a_max = a.iter().max();
    let b_max = b.iter().max();

    if a_max.is_none() {
        *v = b;
    }
    else if b_max.is_none() {
        *v = a;
    }
    else {
        if a_max.unwrap() >= b_max.unwrap() {
            *v = a;
        }
        else {
            *v = b;
        }
    }
}