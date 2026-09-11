pub fn append_total(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    let input_clone = input.clone();
    let mut sum0 = 0;
    let mut sum1 = 0;
    
    for (a,b) in input.into_iter() {
        sum0 = sum0 + a;
        sum1 = sum1 + b;
    }

	// Your for loop logic here
    output.extend(input_clone);
    output.push((sum0, sum1));
    output
}

fn main() {
    let data = vec![(1, 2), (3, 4), (5, 6), (4,8), (6,9)];
    let result = append_total(data);
    println!("{:?}", result); // [(1, 2), (3, 4), (5, 6), (4, 8), (6, 9), (19, 29)]
}