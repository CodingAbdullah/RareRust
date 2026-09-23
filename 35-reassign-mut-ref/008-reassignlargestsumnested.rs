fn main() {
    let v_big = vec![vec![1,2,-5], vec![2,-20,3], vec![1,-1,-1], vec![0, 2, -4]];
    
    let mut t: Vec<i32> = vec![];
    
    assign_to_largest_sum_in_nested(&mut t, v_big);
    
    println!("{:?}", t);
}

pub fn assign_to_largest_sum_in_nested(v: &mut Vec<i32>, v_big: Vec<Vec<i32>>) {
    
    // Index & Sum tracker
    let mut sum_tracker = i32::MIN;

    // Index the vector with the largest sum
    for s in v_big {
        let sum = s.iter().sum();

        if sum >= sum_tracker {
            *v = s;
            sum_tracker = sum;
        }
    }
}