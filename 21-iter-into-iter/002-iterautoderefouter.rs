fn main() {

    let v = &&&&&&&&&&&&&&vec![1,2,3];
    
    let cv: Vec<&i32> = (&&&&&&&&&&&&&&&&&&v).iter().collect();
    println!("{:?}", cv);
}