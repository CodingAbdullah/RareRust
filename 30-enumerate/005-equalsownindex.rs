fn main() {
    let v = vec![0, 2, 2, 3];
		let result = equal_indices(v);

    println!("{:?}", result);
}

pub fn equal_indices(v: Vec<i32>) -> Vec<bool> {
    // your code here
    let mut boolvec = Vec::new();
    
    for (i, value) in v.iter().enumerate() {
        if *value == i as i32 {
            boolvec.push(true);
        }
        else {
            boolvec.push(false);
        }
    }

    boolvec
}