#[derive(Debug, Clone, Copy)]
pub enum Coin {
    Head,
    Tail,
}

fn main() {
    let mut coins = vec![Coin::Head, Coin::Tail, Coin::Tail, Coin::Head];

    invert_all(&mut coins);

    println!("{:?}", coins); // [Tail, Head, Head, Tail]
}

pub fn invert_all(v: &mut Vec<Coin>) {
    for e in v {
        
        *e = match e {
            Coin::Head => Coin::Tail,
            Coin::Tail => Coin::Head,
        }
    }
}
