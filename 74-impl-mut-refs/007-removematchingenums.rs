use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct MyStyles {
    pub styles: HashSet<Style>
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Style {
    Classic,
    Modern,
    Hipster,
    Trendy
}

impl MyStyles {
    pub fn remove_style(&mut self, style: Style) {
        self.styles.remove(&style);
    }
}

fn main() {
    let mut m = MyStyles { styles: HashSet::from([
        Style::Classic,
        Style::Modern,
        Style::Hipster,
        Style::Trendy,
    ]) };
    
    let style = Style::Hipster;
    m.remove_style(style);
    println!("{:?}", m);
}