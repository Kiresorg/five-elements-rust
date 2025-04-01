use std::io;
use std::process;

/// Represents a task with a name and completion status
struct Task {
    name: String,
    completed: bool,
}

impl Task {
    /// Create a new task
    fn new(name: String) -> Self {
        Task { name, completed: false }
    }

    /// Mark the task as complete
    fn complete(&mut self) {
        self.completed = true;
    }
}

/// Entrance Point: The main function where execution starts
fn main() {
    let mut tasks: Vec<Task> = Vec::new(); // Variable: Task list
    loop {
        println!("\nTask Manager");
        println!("1. Add Task");
        println!("2. Mark Task as Complete");
        println!("3. View Tasks");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        // Control Flow: Match statement to handle user input
        match choice {
            "1" => add_task(&mut tasks),
            "2" => mark_task_complete(&mut tasks),
            "3" => view_tasks(&tasks),
            "4" => exit_program(),
            _ => println!("Invalid choice, please try again"),
        }
    }
}

/// Subprogram: Function to add a new task
fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task name:");

    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("Failed to read input");
    let task_name = task_name.trim().to_string();

    if task_name.is_empty() {
        println!("Task name cannot be empty");
        return;
    }

    tasks.push(Task::new(task_name));
    println!("Task added successfully!");
}

/// Subprogram: Function to mark a task as complete
fn mark_task_complete(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available");
        return;
    }

    println!("Enter task number to mark as complete:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {} [{}]", index + 1, task.name, if task.completed { "Done" } else { "Pending" });
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].complete();
            println!("Task marked as complete!");
        } else {
            println!("Invalid task number");
        }
    } else {
        println!("Invalid input");
    }
}

/// Subprogram: Function to view all tasks
fn view_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks available");
        return;
    }

    println!("\nTask List:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {} [{}]", index + 1, task.name, if task.completed { "Done" } else { "Pending" });
    }
}

/// Exit Point: Function to explicitly exit the program
fn exit_program() {
    println!("Exiting Task Manager...");
    process::exit(0);
}
