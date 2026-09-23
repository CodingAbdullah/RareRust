use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)] // Add more derives
pub enum Vote {
    Yes,
    No,
    Abstain,
}

fn main() {
    let votes = vec![
        Vote::Yes,
        Vote::No,
        Vote::Yes,
        Vote::Abstain,
        Vote::Yes,
    ];
    
    let results = count_votes(votes);
    println!("Vote counts: {:?}", results);
}

pub fn count_votes(votes: Vec<Vote>) -> HashMap<Vote, usize> {
    let mut counts = HashMap::new();
    
    for vote in votes {
        // your code here
        if vote == Vote::Yes {
            if !counts.get(&vote).is_none() {
                let value = counts.get(&vote).unwrap();
                counts.insert(vote, value + 1);
            }
            else {
                counts.insert(vote, 1);
            }
        }
        else if vote == Vote::No {
            if !counts.get(&vote).is_none() {
                let value = counts.get(&vote).unwrap();
                counts.insert(vote, value + 1);
            }
            else {
                counts.insert(vote, 1);
            }
        }
        else {
            if !counts.get(&vote).is_none() {
                let value = counts.get(&vote).unwrap();
                counts.insert(vote, value + 1);
            }
            else {
                counts.insert(vote, 1);
            }
        }
    }
    
    counts
}