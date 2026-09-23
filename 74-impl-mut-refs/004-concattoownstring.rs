#[derive(Debug)]
pub struct Message {
    pub text: String
}

// your code here
impl Message {
    pub fn add_to_message(&mut self, concat_text: &str) {
        // Method body
        self.text.push_str(concat_text);
    }
}

fn main() {
    let mut m = Message { text: "Rare".to_string() };
    m.add_to_message("Code");
    println!("{:?}", m); // Message { text: "RareCode" }
}