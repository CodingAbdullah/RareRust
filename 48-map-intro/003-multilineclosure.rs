fn main() {
    /*let mul_of_2_or_3 = |x: i32| {
        if x % 2 == 0 {
            return Some(2);
        } else if x % 3 == 0 {
            return Some(3);
        }
        None
    };
    */

    let result = (0..=10)
        .into_iter()
        .map(|x| {
            if x % 2 == 0 {
               return Some(2);
            }
            else if x % 3 == 0 {
                return Some(3);
            }
            None
        })
        .collect::<Vec<Option<i32>>>();
    println!("{:?}", result);
}
