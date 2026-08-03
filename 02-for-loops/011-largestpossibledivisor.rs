fn main() {
    let x = 12;
    let result = largest_proper_divisor(x);
    println!("{}", result);
  }
  
  pub fn largest_proper_divisor(x: u32) -> u32 {
      let mut gcd = 1;
  
      for i in 1..x {
          if x % i == 0 {
              gcd = i;
          }
      }
  
      gcd
    // loop down to find the largest proper divisor
    // your code here
    // largest proper divisor is defined as the largest number that divides x without leaving a remainder
    // e.g. largest_proper_divisor(12) = 6
    // e.g. largest_proper_divisor(11) = 1
    // e.g. largest_proper_divisor(1) = 1
  
    // assume zero will not be passed because zero does not have a divisor
  
  
    
  } 