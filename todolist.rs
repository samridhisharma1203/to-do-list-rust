use std::io;
use std::io::Write;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("To-Do List:");
        list_tasks(&tasks);

        println!("Menu:");
        println!("1. Add Task(s)");
        println!("2. Complete Task");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => add_tasks(&mut tasks),
            2 => complete_task(&mut tasks),
            3 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn list_tasks(tasks: &Vec<String>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}

fn add_tasks(tasks: &mut Vec<String>) {
    let mut num_tasks = String::new();
    print!("How many tasks would you like to add: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut num_tasks).expect("Failed to read line");
    let num_tasks: u32 = num_tasks.trim().parse().expect("Invalid input");

    for _ in 0..num_tasks {
        let mut task = String::new();
        print!("Enter a new task: ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut task).expect("Failed to read line");
        tasks.push(task.trim().to_string());
    }

    println!("Tasks added!");
}

fn complete_task(tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks to complete.");
        return;
    }

    list_tasks(tasks);

    let mut choice = String::new();
    print!("Enter the number of the task you want to complete: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: usize = choice.trim().parse().expect("Invalid input");

    if choice > 0 && choice <= tasks.len() {
        tasks.remove(choice - 1);
        println!("Task completed!");
    } else {
        println!("Invalid task number. Please try again.");
    }
}
