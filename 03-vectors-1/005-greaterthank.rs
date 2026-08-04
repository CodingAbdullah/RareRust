fn main() {
	let my_vec = vec![1,2,3];
	let k = 0;
	let result = greater_than_k(my_vec, k);
	
	println!("{}", result);
}

pub fn greater_than_k(v: Vec<u32>, k: u32) -> bool {
  // your code here
  for i in 0..v.len() {
    if v[i] > k {
        continue;
    }
    else {
        return false;
    }
  }

  true
} 