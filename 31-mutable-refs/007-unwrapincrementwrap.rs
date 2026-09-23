fn main() {
	let mut v = vec![Some(1), Some(2), None, Some(3)];
	
	unwrap_inc_wrap(&mut v);
	println!("{:?}", v);
}

pub fn unwrap_inc_wrap(v: &mut Vec<Option<i32>>) {

	for i in 0..v.len(){
        if v[i].is_some(){
            let value = Some(v[i].unwrap() + 1);
            v[i] = value;
        }
    }
}