fn main() {
    let a = 1;
    let b = 2;
    let c = 3;

    let numbers: Vec<&i32> = vec![&a, &b, &c];

   let values = collect_values(numbers);
    println!("{:?}", values);
}

pub fn collect_values(input: Vec<&i32>) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    for value in input.into_iter() {
        // dereference each value and push it to values
        // your code here
        values.push(*value);
    }

    values
}
