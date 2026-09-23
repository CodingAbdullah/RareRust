use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, u64> = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);
    m.insert("c", 3);

    println!("{:?}", values_to_array3(&m)); // Some([?, ?, ?]) order unspecified

    m.insert("d", 4);
    println!("{:?}", values_to_array3(&m)); // None
}

pub fn values_to_array3(map: &HashMap<&str, u64>) -> Option<[u32; 3]> {
    // your code here
    let values_vec_u64: Vec<u64> = map.values().map(|x| {*x}).collect();
    let mut values_vec_u32: Vec<u32> = Vec::new();

    if values_vec_u64.len() != 3 {
        return None;
    }
    else {
        for item in values_vec_u64 {
            let result: Result<u32, _> = u32::try_from(item);

            let matched_result = match result {
                Ok(x) => Some(x),
                Err(_) => None,
            };

            if matched_result.is_none() {
                return None;
            }
            else {
                values_vec_u32.push(matched_result.unwrap());
            }
        }

        match values_vec_u32.try_into() {
            Ok(arr) => Some(arr),
            Err(_) => None,
        }
    }
}
