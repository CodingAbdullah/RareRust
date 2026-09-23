fn main() {
    let mut a = [1,2,5,4,3];
    
    overwrite(&mut a);
    println!("{:?}", a);
}

pub fn overwrite(a: &mut [i32; 5]) {
   // *a = [0,1,2,3,4];

   for (i, e) in a.iter_mut().enumerate() {
    *e = i as i32;
   }
}