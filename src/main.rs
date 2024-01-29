use std::io;
#[derive(Clone)]
struct Task {
    id: usize,
    desc: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, desc: String) -> Task {
        Task {
            id,
            desc,
            completed: false,
        }
    }
}

fn add_task(lis: &mut Vec<Task>, desc: String) -> Task {
    let new_task = Task::new(lis.len() + 1, desc);
    lis.push(new_task.clone());
    new_task
}

fn complete(lis: &mut Vec<Task>, id: usize) -> Option<&Task> {
    if id > lis.len() || id == 0 {
        return None;
    }
    lis[id - 1].completed = true;
    Some(&lis[id - 1])
}

fn list_tasks(lis: &Vec<Task>) {
    for i in lis {
        println!("{} {} {}", i.id, i.desc, i.completed);
    }
}

fn main() {
    let mut lis: Vec<Task> = Vec::new();

    loop {
        let mut option = String::new();
        println!("1. Add Task \n2. Complete Task \n3. Show All Tasks \n4. Exit");
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: usize = option.trim().parse().expect("Invalid input");

        match option {
            1 => {
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).expect("Failed to read line");
                let add = add_task(&mut lis, desc.trim().to_string());
            }
            2 => {
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = id.trim().parse().expect("Invalid input");
                match complete(&mut lis, id) {
                    Some(task) => println!("Task completed: {} {} {}", task.id, task.desc, task.completed),
                    None => println!("Invalid task ID"),
                }
            }
            3 => list_tasks(&lis),
            _ => break,
        }
    }
}
