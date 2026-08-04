fn main() {
	let mut v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result); // [3,4,5,99]
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
  // make a copy
  let mut my_vec = vec![];

  for i in 0..v.len() {
	  my_vec.push(v[i]);
  }
  
  // make the change
  my_vec.push(99);
  my_vec
} 