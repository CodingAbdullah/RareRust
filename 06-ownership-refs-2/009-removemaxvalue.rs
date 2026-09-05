fn main() {
	let v = vec![1,2,3];
	
	let result = remove_max(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn remove_max(v: &Vec<u32>) -> Vec<u32> {
	// your code here
    let mut max = 0;

    // If length of vector is empty or 1 return empty
    if v.len() == 0 || v.len() == 1 {
        return Vec::new();
    }
    else {
        // Find max value in array
        for i in 0..v.len() {
            if v[i] >= max {
                max = v[i];
            }
        }

        let mut new_vec = Vec::new();
        let mut flag = false;

        for i in 0..v.len() {
            if v[i] == max && !flag {
                flag = true;
            }
            else {
                new_vec.push(v[i]);
            }
        }

        return new_vec;
    }
} 