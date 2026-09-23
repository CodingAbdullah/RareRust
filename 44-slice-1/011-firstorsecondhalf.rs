fn main() {
    let v = vec![1, 2, 1];
    let result = first_or_second_half(&v);
    println!("Result: {}", result);
}

pub fn first_or_second_half(sl: &[i32]) -> u8 {
    if sl.len() == 0 {
        return 0;
    }
    else if sl.len() == 1 {
        return 0;
    }
    else {
        let halfwaypoint = sl.len()/2;
        let max_value = *sl.iter().max().unwrap();
        let firsthalfslice = &sl[0..halfwaypoint];

        if firsthalfslice.contains(&max_value) {
            return 0;
        }
        else {
            return 1;
        }
    }
}
