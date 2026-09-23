fn main() {
	let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
	
	transpose(&mut matrix);
	println!("{:?}", matrix);
}

// put fn transpose ... your code here
pub fn transpose(array: &mut [[i32; 3]; 3]) {
    let clonearray = array.clone();

    // transpose is basically clonearray[i][j] == array[j][i]
    for i in 0..3 {
        for j in 0..3 {
            array[j][i] = clonearray[i][j];
        }
    }
}