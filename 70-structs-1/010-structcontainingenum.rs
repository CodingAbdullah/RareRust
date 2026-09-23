#[derive(PartialEq)]
pub enum Status {
    NotStarted,
    InProgress,
    Done,
}

pub struct Task {
    pub status: Status,
    pub description: String
}

fn main() {
    let tasks = vec![
        Task { status: Status::NotStarted, description: "clean the windows".to_string() },
        Task { status: Status::Done, description: "clean the floors".to_string() },
        Task { status: Status::InProgress, description: "clean the dishes".to_string() }
    ];
    
    let result = count_by_status(tasks, Status::InProgress);
    println!("{}", result);
}

pub fn count_by_status(tasks: Vec<Task>, status: Status) -> u32 {
    tasks.into_iter()
    .map(|x| {
        if x.status == status {
            return 1;
        }
        else {
            return 0;
        }
    })
    .sum()
    
}