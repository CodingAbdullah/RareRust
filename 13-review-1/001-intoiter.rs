pub fn check_even(v: Vec<u32>) -> Vec<(u32, bool)> {
    // your code here
    let mut vec_tuple: Vec<(u32, bool)> = Vec::new();

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            vec_tuple.push((v[i], true));
        }
        else {
            vec_tuple.push((v[i], false));
        }
    }

    return vec_tuple;
}

fn main() {
    let numbers = vec![0, 1, 2, 3]; // Using 'numbers' as per function call in problem
    let result = check_even(numbers);
    println!("{:?}", result); // [(0, true), (1, false), (2, true), (3, false)]
}
