fn main() {
	let v = vec![1,2,3];
	
	let result = is_sorted(v);
	println!("{}", result);
}

pub fn is_sorted(v: Vec<i32>) -> bool {
    // your code here
    if v.len() == 1 {
        return true;
    }
    else if v.len() == 2 {
        if v[0] <= v[1] {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        for i in 1..v.len() {
            if v[i-1] <= v[i] {
                continue;
            }
            else {
                return false;
            }
        }
    }
    
    return true;
} 