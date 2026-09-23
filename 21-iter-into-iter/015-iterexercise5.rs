fn main() {
    let v = &&vec![&1,&2,&3];
    
    let _cv: Vec<&&i32> = v.iter().collect();
    println!("{}", "success!");
}