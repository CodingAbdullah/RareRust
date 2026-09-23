fn main() {
	let v = vec![72057594037927936, 281474976710656, 131072];
	
	let result = clamp_and_cut(v);
	println!("{:?}", result); // [2147483647, 2147483647, 65536]
}

pub fn clamp_and_cut(mut v: Vec<u64>) -> Vec<u64> {
		// your code here
        let max: u64 = u32::MAX as u64;

        for item in &mut v {
            if *item > max {
                *item = max;
            }
        }

        for item_2 in &mut v {
            *item_2 = *item_2 / 2;
        }
        
        v
}