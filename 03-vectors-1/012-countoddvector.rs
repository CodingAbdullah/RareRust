fn main() {
	let v = vec![1,2,3,4,5];
	
	let result = count_odds(v);
	println!("{}", result);
}

pub fn count_odds(v: Vec<i32>) -> i32 {
    // your code here
    let mut count = 0;

    for i in 0..v.len() {
        if v[i] % 2 != 0 {
            count = count + 1;
        }
        else {
            continue;
        }
    }
    
    return count;
} 