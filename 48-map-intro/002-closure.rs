fn main() {
    // let increment = |x: i32| -> i32 { x + 1 };
 
     let v = vec![1, 2, 3, 4, 5];
     let result = v.into_iter().map(|x| { x+1 }).collect::<Vec<i32>>();
     println!("{:?}", result);
 }
 