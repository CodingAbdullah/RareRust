fn main() {
    let a = [1, 2, 3, 2, 1];
    let result = up_to_max(&a);
    println!("{:?}", result);
}

pub fn up_to_max(a: &[i32]) -> Vec<i32> {
    // your code here
    if a.len() == 0 {
        return vec![];
    }
    else {
        let maxv = *a.iter().max().unwrap();
        let mut newvec: Vec<i32> = Vec::new();

        let mut ithindex = 0;

        for (i, e) in a.iter().enumerate() {
            if *e == maxv {
                ithindex = i;
                break;
            }
        }

        for j in 0..=ithindex {
            newvec.push(a[j]);
        }

        newvec
    }
}
