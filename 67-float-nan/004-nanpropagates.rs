fn main() {
    let x = 1.0;
let result = nan_add(x);
println!("{}", result);
}

pub fn nan_add(x: f32) -> f32 {
// your code here
f32::NAN + x
}

