// your code here
#[derive(Debug, PartialEq, Eq)]
pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    // your code here
    pub fn new() -> Stack {
        Stack { items: vec![] }
    }

    pub fn from(sli: &[i32]) -> Stack {
        Stack { items: Vec::from(sli) }
    }

    pub fn push(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.items.len() == 0 {
            return None;
        }
        else {
            let item: i32 = self.items.pop().unwrap();
            return Some(item);
        }
    }

    pub fn peek(&self) -> Option<i32> {
        if self.items.len() == 0 {
            return None;
        }
        else {
            let items_length = self.items.len();

            let peek_item = self.items[items_length - 1];

            return Some(peek_item);
        }
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }
}

fn main() {
    let mut stack1 = Stack::new();
    let stack2 = Stack::from(&[1, 2, 3]);

    stack1.push(1);
    stack1.push(2);
    stack1.push(3);

    println!("{}", stack1 == stack2);
    stack1.pop();
    println!("{}", stack1 == stack2);
    println!("{:?}", stack2.peek());
}