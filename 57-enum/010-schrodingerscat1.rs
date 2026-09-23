// your code here
pub enum SchrodingersCat {
    Alive,
    Dead,
    Superposition,
}
fn main() {
    let mut cat = SchrodingersCat::Alive;

    put_in_box(&mut cat);
    println!("cat state: {}", cat_state(&cat));
}

pub fn put_in_box(cat: &mut SchrodingersCat) {
    match cat {
        _ => *cat = SchrodingersCat::Superposition
    }
}

pub fn cat_state(cat: &SchrodingersCat) -> String {
    match *cat {
        SchrodingersCat::Alive => "alive".into(),
        SchrodingersCat::Dead => "dead".into(),
        SchrodingersCat::Superposition => "superposition".into(),
    }
}
