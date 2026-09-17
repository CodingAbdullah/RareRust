fn main() {
    let v = vec![1,2,3];
    
    for e in v.iter() {
      println!("{}", e);
    }
    
    println!("v is not consumed {:?}", v);
  }