fn main() {

    enum Color {
        // your code here
        Red,
        Green,
        Blue
    }
    
    let c = Color::Green;
    
    let str_value = match c {
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue"
    };
    
    println!("{}", str_value);
}