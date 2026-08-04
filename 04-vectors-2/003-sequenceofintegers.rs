fn main() {
    let result = simple_count(6);
    println!("{:?}", result);
  }
  
  pub fn simple_count(n: u32) -> Vec<u32> {
      // your code here
      let mut v = vec![];
  
      if n == 0 {
          return v;
      }
      else {
          for i in 1..n+1 {
              v.push(i);
          }
      }
  
      return v;
  } 