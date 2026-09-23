fn main() {
    
    let v: Vec<i32> = vec![];
    
    let a: Vec<i32> = vec![1000];
    let b: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    
    let result = assign_to_largest_sum(v, a, b);
    println!("{:?}", result);
}

pub fn assign_to_largest_sum(mut v: Vec<i32>, a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // your code here
    let a_sum: i32 = a.iter().sum();
    let b_sum: i32 = b.iter().sum();

    if a_sum >= b_sum {
        v = a;
        return v;
    }
    else {
        v = b;
        return v;
    }



}