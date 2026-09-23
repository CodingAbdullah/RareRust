fn main() {
	let mut a = [[1,1,1,1],[2,2,2,0],[3,3,3,3],[0,0,4,4]];
	
	row_to_zero(&mut a);
	println!("{:?}", a);
}

pub fn row_to_zero(a: &mut [[i32; 4]; 4]) {
	// your code here
    for i in 0..4 {
        if a[i].contains(&0) {
            a[i][0] = 0;
            a[i][1] = 0;
            a[i][2] = 0;
            a[i][3] = 0;
        }
    }
}