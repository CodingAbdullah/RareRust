fn main() {
	let mut v = vec![(true, false), (false, true), (false, false)];
	
	reverse_all(&mut v);
	println!("{:?}", v); // [(false, true), (true, false), (false, false)]
}

pub fn reverse_all(v: &mut Vec<(bool, bool)>) {
    // your code here

    for t in v {
        if t.0 == t.1 {
            break;
        }
        if t.0 == true {
            t.0 = false;
        }
        else {
            t.0 = true;
        }

        if t.1 == true {
            t.1 = false;
        }
        else {
            t.1 = true;
        }
    }
}