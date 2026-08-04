fn main() {
    let result = countdown(6);
    println!("{:?}", result);
  }
  
  pub fn countdown(n: u32) -> Vec<u32> {
      
      // your code here
      let mut v = vec![];
  
      if n == 0 {
          return v;
      }
      else {
          for i in (0..n).rev() {
              v.push(i);
          }   
      }
  
      return v;
      
  }