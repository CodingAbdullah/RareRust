fn main() {
    let v = vec![&1, &2, &3, &4, &5];
    let result = slice_of_ref_to_vec(&v);
    println!("{:?}", result); // Output: [1, 2, 3, 4, 5]
}

pub fn slice_of_ref_to_vec(a: &[&i32]) -> Vec<i32> {
		// your code here
        let mut newvec: Vec<i32> = Vec::new();

        for i in 0..a.len() {
            newvec.push(*a[i]);
        }

        newvec
}