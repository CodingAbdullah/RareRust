fn main() {
    
    let mut a: [i32; 3] = [1,2,3];
    
    let result = sum(a);
    println!("{}", result);
    
    inc_all(&mut a); // [2,3,4];
	 println!("{:?}", a);
    
}

// add one to each element
pub fn inc_all(array: &mut [i32; 3]) {
    for i in 0..array.len() {
        array[i] = array[i] + 1;
    }
}

pub fn sum(a: [i32; 3]) -> i32 {
    let mut sum = 0;

    for i in 0..a.len() {
        sum += a[i];
    }

    sum
}