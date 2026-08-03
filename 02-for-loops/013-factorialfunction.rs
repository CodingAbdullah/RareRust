fn main() {
    let n = 5;
    let result = factorial(n);
    println!("{}", result);
  }
  
  pub fn factorial(n: u32) -> u32 {
    // your code 
    let mut finalresult = 1;
    let mut factorialboundary = n;
  
    while factorialboundary >= 1 {
      finalresult = finalresult*factorialboundary;
      factorialboundary = factorialboundary - 1;
    }
  
    finalresult
  
  } 