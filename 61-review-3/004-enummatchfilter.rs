#[derive(Debug, Clone, Copy)]
pub enum Status {
    Active(u32),
    Expired(u32),
}

fn main() {
    let v = Vec::<Status>::from([
        Status::Active(100),
        Status::Expired(60),
        Status::Active(204),
        Status::Expired(59),
    ]);
    let result = only_active(v);
    println!("{:?}", result); // [Active(100), Active(204)]
}

pub fn only_active(v: Vec<Status>) -> Vec<Status> {
    // your code here
    v.iter()
    .filter(|&x| !matches!(x, Status::Expired(_))
    )
    .map(|y| *y)
    .collect()
}
