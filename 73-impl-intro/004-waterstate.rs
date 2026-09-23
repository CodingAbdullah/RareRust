pub struct Water {
    pub volume: u32,
    pub temperature: i32,
}

// your code here
impl Water {
    pub fn is_boiling(&self) -> bool {
        if self.temperature < 100 {
            return false;
        }
        else {
            return true;
        }
    }

    pub fn is_frozen(&self) -> bool {
        if self.temperature <= 0 {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn empty(&self) -> bool {
        if self.volume == 0 {
            return true;
        }
        else {
            return false;
        }
    }
}

fn main() {
    let w = Water { volume: 1, temperature: 50 };

    let ice = w.is_frozen();
    let steam = w.is_boiling();
    let empty = w.empty();

    println!("{} {} {}", ice, steam, empty);
}

