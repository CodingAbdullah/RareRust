use std::collections::HashSet;

fn main() {
  let v = vec![1, 2, 2, 3, 4, 4, 4, 5, 1];
  let deduped_v = dedup(v);
  println!("{:?}", deduped_v);
}

pub fn dedup(v: Vec<i32>) -> Vec<i32> {
  // Complete this
  let setconvert: HashSet<i32> = v.into_iter().collect();
  let vectorconvert: Vec<i32> = setconvert.into_iter().collect();

  vectorconvert

}
