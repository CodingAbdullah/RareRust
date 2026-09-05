fn main() {
    let result_true = can_downcast(127);
    println!("{}", result_true);
    let result_false = can_downcast(128);
    println!("{}", result_false);
    let result_true_neg = can_downcast(-128);
    println!("{}", result_true_neg);
    let result_false_neg = can_downcast(-129);
    println!("{}", result_false_neg);
}

pub fn can_downcast(x: i32) -> bool {
	if x <= i8::MAX as i32 && x >= i8::MIN as i32 { // cast here
		true
	} else {
		false
	}
} 