#[derive(Debug)]
pub struct Message {
    pub text: String,
}

// your code here
impl Message {
    pub fn combine_message(&self, add_text: &Message) -> Message {
        // Method body
        let new_text = self.text.clone() + &add_text.text;
        
        Message {
            text: new_text
        }
    }
}

fn main() {

    let m1 = Message { text: "Rare".to_string() };
    let m2 = Message { text: "Code".to_string() };
    let result = m1.combine_message(&m2);
    println!("{:?}", result);
}
