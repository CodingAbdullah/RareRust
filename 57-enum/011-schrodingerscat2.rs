pub enum SchrodingersCat {
    Alive,
    Dead,
    Superposition,
}

fn main() {
    let mut cat = SchrodingersCat::Alive;

    put_in_box(&mut cat);
    println!("cat state: {}", cat_state(&cat));

    open_box(&mut cat, true);
    println!("cat state: {}", cat_state(&cat));
}

pub fn put_in_box(cat: &mut SchrodingersCat) {
    *cat = SchrodingersCat::Superposition;
}

pub fn cat_state(cat: &SchrodingersCat) -> String {
    match *cat {
        SchrodingersCat::Alive => "alive".into(),
        SchrodingersCat::Dead => "dead".into(),
        SchrodingersCat::Superposition => "superposition".into(),
    }
}

pub fn open_box(cat: &mut SchrodingersCat, decision: bool) {
    // your code here
    match decision {
        true => *cat = SchrodingersCat::Alive,
        false => *cat = SchrodingersCat::Dead
    }
}