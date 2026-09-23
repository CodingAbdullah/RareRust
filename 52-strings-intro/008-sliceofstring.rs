fn main() {
    let s = vec!['R', 'a', 'r', 'e', 'C', 'o', 'd', 'e'].into_iter().collect::<String>();
 
	let slice_of_string1: &str = &s[..4];
	let slice_of_string2: &str = "practice rust";

	println!("{}", slice_of_string1);
	println!("{}", slice_of_string2);
}
