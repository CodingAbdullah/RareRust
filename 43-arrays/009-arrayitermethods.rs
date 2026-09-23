fn main() {
    let a = [1,2,5,4,3];
    
    let result = coverage(a);
    println!("{}", result); // 4
}

pub fn coverage(a: [i32; 5]) -> i32 {
	// your code here
    let min: i32 = a.into_iter().min().unwrap();
    let max: i32 = a.into_iter().max().unwrap();

    max - min
}