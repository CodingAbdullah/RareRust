fn main() {
    
    let a: [i32; 3] = [1,2,3];
    
    // none of these take ownership of `a`
    take(a);
    let a1 = a;
    // your code here
    let a2 = a;
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn take(_a: [i32; 3]) {}