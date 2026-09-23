#[derive(Debug)]
pub struct Restaurant {
   pub name: String,
   pub stars: u8,
}

fn main() {
    
    let v = vec![
        Restaurant { name: "cookhouse".to_string(), stars: 4 },
        Restaurant { name: "infinitecoffee".to_string(), stars: 5 },
        Restaurant { name: "pastaden".to_string(), stars: 3 },
        Restaurant { name: "aliceandbobcafe".to_string(), stars: 2 },
    ];
    
    let result = at_least_n_stars(v, &4);
    println!("{:?}", result);
}

pub fn at_least_n_stars(v: Vec<Restaurant>, stars: &u8) -> Vec<Restaurant> {
    // your code here
    v.into_iter()
    .filter(|r| {
        r.stars >= *stars
    })
    .collect()
}