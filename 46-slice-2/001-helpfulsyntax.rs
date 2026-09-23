fn main() {
    let a = [1,2,3,3,4,5];

    let result = up_to_including_k(&a, 3);
    println!("{:?}", result); // Output: [1, 2, 3]
}

pub fn up_to_including_k(a: &[i32], k: i32) -> Vec<i32> {
    let mut ithindex: i32 = -1;
    let mut newvec: Vec<i32> = Vec::new();

    for (i, e) in a.iter().enumerate() {
        if *e == k {
            ithindex = i as i32;
            break;
        }
    }

    if ithindex == - 1 {
        return vec![];
    }
    else {
        let usizeindex = ithindex as usize;

        for i in 0..=usizeindex {
            newvec.push(a[i]);
        }
    }
    newvec
}