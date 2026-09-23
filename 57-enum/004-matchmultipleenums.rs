// your enum here
pub enum PrimaryColor {
    Red,
    Blue,
    Green
}

pub enum MixedColor {
    Cyan,
    Magenta,
    Yellow,
    Red,
    Green,
    Blue,
}

fn main() {
    let color_1 = PrimaryColor::Red;
    let color_2 = PrimaryColor::Green;

    
    let result = mix_colors(color_1, color_2);
    let mix = match result {
        MixedColor::Magenta => "magenta",
        MixedColor::Yellow => "yellow",
        MixedColor::Cyan => "cyan",
        MixedColor::Red => "red",
        MixedColor::Green => "green",
        MixedColor::Blue => "blue",
    };
    
    println!("{}", mix);
}

pub fn mix_colors(c1: PrimaryColor, c2: PrimaryColor) -> MixedColor {
    match (c1, c2) {
        (PrimaryColor::Red, PrimaryColor::Blue) => MixedColor::Magenta,
        (PrimaryColor::Blue, PrimaryColor::Red) => MixedColor::Magenta,
        (PrimaryColor::Red, PrimaryColor::Green) => MixedColor::Yellow,
        (PrimaryColor::Green, PrimaryColor::Red) => MixedColor::Yellow,
        (PrimaryColor::Blue, PrimaryColor::Green) => MixedColor::Cyan,
        (PrimaryColor::Green, PrimaryColor::Blue) => MixedColor::Cyan,
        (PrimaryColor::Red, PrimaryColor::Red) => MixedColor::Red,
        (PrimaryColor::Green, PrimaryColor::Green) => MixedColor::Green,
        (PrimaryColor::Blue, PrimaryColor::Blue) => MixedColor::Blue,
    }
}