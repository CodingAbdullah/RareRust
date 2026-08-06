fn main() {
	let v = vec![1,1,5];
	let cond1 = all_elements_less_than_k(&v, 6); // true
	let cond2 = sum_greater_than_s(&v, 6); // true
	println!("{}", cond1 && cond2);
}

// pub fn all_elements_less_than_k
pub fn all_elements_less_than_k(v1: &Vec<i32>, k: i32) -> bool {
    let mut flag = true;

    for i in 0..v1.len() {
        if v1[i] >= k {
            flag = false;
            break;
        }
    }

    return flag;
}

// pub fn sum_greater_than_s
pub fn sum_greater_than_s(v1: &Vec<i32>, s: i32) -> bool {
    let mut sum = 0;

    for i in 0..v1.len() {
        sum = sum + v1[i];
    }

    if sum > s {
        return true;
    }
    else {
        return false;
    }
}