pub fn clone_filter_update(input: &Vec<(i32, bool)>) -> Vec<(i32, bool)> {
    let cloned = input.clone();
    let mut result = Vec::new();

		// Your logic here
        for (a,b) in cloned.into_iter() {
            if a % 2 == 0 {
                let new_b = !b;
                result.push((a,new_b));
            }
        }

    result
}

fn main() {
    let input = vec![(2, false), (3, false), (4, false)];
    println!("Original: {:?}, Filtered: {:?}", &input, clone_filter_update(&input) );
}