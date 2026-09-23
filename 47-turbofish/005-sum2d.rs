fn main() {
	let a = [[1,2,3],[4,5,6],[7,8,9]];
	
	let result = two_d_sum(&a);
	
	println!("{}", result); // 45
}

pub fn two_d_sum(a: &[[i32; 3]; 3]) -> i32 {
    let mut sum_calc = 0;

    for item in a.iter() {
        sum_calc = item.iter().sum::<i32>() + sum_calc;
    }

    sum_calc
}