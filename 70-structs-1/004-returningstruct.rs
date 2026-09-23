#[derive(Debug)]
pub struct Account {
    pub account_number: u32,
    pub balance: u32,
    pub owner: String,
}

fn main() {
    let result = create_account(0, 0, "Bob");
    println!("{:?}", result);
}

pub fn create_account(account_number: u32, balance: u32, owner: &str) -> Account {
    let new_account = Account {
        account_number: account_number,
        balance: balance,
        owner: String::from(owner)
    };

    new_account
}