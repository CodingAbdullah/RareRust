pub enum Outcomes {
    Player1,
    Player2,
    Tie
}

pub enum Items {
    Rock,
    Paper,
    Scissors
}

fn main() {
    let player_1_choice = Items::Rock;
    let player_2_choice = Items::Paper;
    
    let result = play_game(player_1_choice, player_2_choice);
    match result {
        Outcomes::Player1 => println!("player 1 wins"),
        Outcomes::Player2 => println!("player 2 wins"),
        Outcomes::Tie => println!("tie"),
    };
}

pub fn play_game(playerone_choice: Items, playertwo_choice: Items) -> Outcomes {
    // your code here
    let item_tuple: (Items, Items) = (playerone_choice, playertwo_choice);

    match item_tuple {
        (Items::Rock, Items::Scissors) => Outcomes::Player1,
        (Items::Rock, Items::Paper) => Outcomes::Player2,
        (Items::Rock, Items::Rock) => Outcomes::Tie,
        (Items::Scissors, Items::Rock) => Outcomes::Player2,
        (Items::Scissors, Items::Paper) => Outcomes::Player1,
        (Items::Scissors, Items::Scissors) => Outcomes::Tie,
        (Items::Paper, Items::Scissors) => Outcomes::Player2,
        (Items::Paper, Items::Rock) => Outcomes::Player1,
        (Items::Paper, Items::Paper) => Outcomes::Tie,
    }
}