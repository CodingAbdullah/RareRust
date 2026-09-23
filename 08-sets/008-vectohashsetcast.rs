use std::collections::HashSet;

fn main() {
    let v1 = vec![10i64, -20i64, 500i64, i32::MAX as i64, i32::MIN as i64];
    println!("Original i64: {:?}", v1);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v1));

    let v2 = vec![i32::MAX as i64 + 1, i32::MIN as i64 - 1];
    println!("Original i64: {:?}", v2);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v2));

    let v3: Vec<i64> = vec![];
    println!("Original i64: {:?}", v3);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v3));
}

// Casts a vector of i64 to a HashSet of i32, skipping values that don't fit.
pub fn safe_cast_vec_to_set(v: &Vec<i64>) -> HashSet<i32> {
    let mut set = HashSet::new();

    for i in 0..v.len() {
        if v[i] >= i32::MIN as i64 && v[i] <= i32::MAX as i64 {
            set.insert(v[i] as i32);
        }
    }

    return set;
} 