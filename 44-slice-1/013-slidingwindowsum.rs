fn main() {
    let a = [1, 2, 3, 4, 1, 2];
    let k = 3;
    let target = 8;

    let result = find_region(&a, k, target);
    println!("{:?}", result);
}

pub fn find_region(sl: &[i32], k: usize, target: i32) -> Option<usize> {
    // your code here
    if k == 0 {
        return None;
    }
    else if k > sl.len() {
        return None;
    }
    else {
        let mut ithindex = 0;

        while ithindex + k <= sl.len() {
            let tempslc = &sl[ithindex..k+ithindex];

            let tempsum: i32 = tempslc.iter().sum();

            if tempsum == target {
                return Some(ithindex as usize);
            }

            ithindex = ithindex + 1;
        }
    }
    return None;
}
