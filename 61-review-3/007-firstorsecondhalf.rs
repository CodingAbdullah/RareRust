use std::collections::HashSet;

pub enum Half {
    First,
    Second,
}

fn main() {
    let a = [&1, &2, &3];

    let result = which_half(&a, Half::Second);
    println!("{:?}", result);
}

pub fn which_half(sl: &[&i32], which: Half) -> HashSet<i32> {
		// your code here
        let mut new_set: HashSet<i32> = HashSet::new();

        let sl_length = sl.len();
        
        if sl_length == 0 {
            return new_set;
        }
        else {
            match which {
                Half::First => {
                    for idx in 0..sl_length/2 {
                        new_set.insert(*sl[idx]);
                    }
                },
                Half::Second => {
                    for idx in sl_length/2..sl_length {
                        new_set.insert(*sl[idx]);
                    }
                },
            }
            return new_set;
        }
}
