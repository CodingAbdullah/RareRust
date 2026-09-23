fn main() {
    let a = [1,2,3,3,4,5];

    let result = k_to_end(&a, 3);
    println!("{:?}", result); // Output: [3, 4, 5]
}

pub fn k_to_end(a: &[i32], k: i32) -> Vec<i32> {
    // your code here
    let mut ithindex: i32 = -1;
    let mut newvec: Vec<i32> = Vec::new();

    // Find the latest index
    for (i, e) in a.iter().enumerate() {
        if *e == k {
            ithindex = i as i32;
        }
    }

    if ithindex == -1 {
        return vec![];
    }
    else {
        let usizeindex: usize = ithindex as usize;

        for i in usizeindex..a.len() {
            newvec.push(a[i]);
        }
    }
    
    newvec
}