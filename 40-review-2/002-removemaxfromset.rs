use std::collections::HashSet;

fn main() {
	let mut s = HashSet::from([1,2,3]);
	
	remove_max(&mut s);
	println!("{:?}", s);
}

pub fn remove_max(s: &mut HashSet<i32>) {
    let max: Option<&i32> = s.iter().max();

    if max.is_none() {
        return;
    }
    else {
        let mut setvec: Vec<i32> = s.iter().copied().collect();

        for i in 0..setvec.len() {
            if setvec[i] == *max.unwrap() {
                setvec.remove(i);
            }
        }

        *s = setvec.iter().copied().collect();
    }
}