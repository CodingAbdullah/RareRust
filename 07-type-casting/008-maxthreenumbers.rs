fn main() {
	let x = 3;
	let y = 5;
	let z = 4;
	
	let result = max_of_three(x, y, z);
	println!("{}", result);
}

pub fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
	// let m = if... your code here
    let m = if x == y && y == x {
        x
    }
    else if x >= y && x >= z {
        x
    }
    else if y >= x && y >= z {
        y
    }
    else if z >= x && z >= y {
        z
    }
    else {
        x
    };
    m
} 