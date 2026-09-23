fn main() {
    let v = vec![1,2,3];
    
    let result = v.get(7); // out of bounds
    
    if result.is_none() {
        println!("{}", "No value");
    }

    if !result.is_none() {
        println!("Value: {:?}", result);
    }
}