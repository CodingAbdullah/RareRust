fn main() {
    let v = vec![1,2,3]; // Made immutable
    
    let result = v.get(0);
    
    if !result.is_none() {
	    let sum = result.unwrap() + 1; // replace result with result.unwrap()
        println!("{}", sum);
    }
} 