fn main() {
    let a = 10;
    let b = 20;
    let c = 30;

    let refs = vec![&a, &b, &c];

    let result = return_owned_vector(&refs);
    println!("{:?}",result);
}

pub fn return_owned_vector(input: &Vec<&i32>) -> Vec<i32> {
    let v = dereference_values_from_vector(input.clone()); // Fix this
    v
}

pub fn dereference_values_from_vector(v: Vec<&i32>) -> Vec<i32> {
    let mut values = Vec::new();

    for val in v {
        values.push(*val); // Fix this
    }
    values
}
