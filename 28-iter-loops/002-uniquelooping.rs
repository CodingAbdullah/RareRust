fn main() {
    let v = vec![1,2,3];
    
    for e in (&v).into_iter() {
      accept(e);
    }
    
    println!("v is not consumed {:?}", v);
  }
  
  fn accept(elo: &i32) {}