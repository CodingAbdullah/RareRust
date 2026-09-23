fn main() {
    let t = (vec![1, 2, 3], vec![4, 5]);
    println!("{:?}", sum_tuple(&t));
}

pub fn sum_tuple(input: &(Vec<i32>, Vec<i32>)) -> (i32, i32) {
	// your code here
    let mut sumone = 0;
    let mut sumtwo = 0;

    for i in 0..input.0.len() {
        sumone = sumone + input.0[i];
    }

    for i in 0..input.1.len() {
        sumtwo = sumtwo + input.1[i];
    }

    (sumone, sumtwo)
}
