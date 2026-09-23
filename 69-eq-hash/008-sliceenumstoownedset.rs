use std::collections::HashSet;

// your code here
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub fn deduplicate_priorities(priorities: &[Priority]) -> HashSet<Priority> {
    // your code here
    let mut priority_set: HashSet<Priority> = HashSet::new();

    for priority in priorities {
        priority_set.insert(*priority);
    }

    priority_set
}

fn main() {
    let tasks = vec![
        Priority::High,
        Priority::Low,
        Priority::High,
        Priority::Critical,
        Priority::Low,
    ];
    
    let unique_priorities = deduplicate_priorities(&tasks);
    println!("Original still exists: {:?}", tasks);
    println!("Unique priorities: {:?}", unique_priorities);
}