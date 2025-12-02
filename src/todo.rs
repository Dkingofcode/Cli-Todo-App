// Todo.rs

// A simple CLI Todo App module
// Implement Task struct
// Implement functions to add, remove, list, and mark tasks as completed
// Use Vectors to store the list of tasks
// Use Enums to define task status
// Use Traits to define shared behaviour for tasks
// Use HashMaps to categorize tasks by project or priority
// Use Modules to organize the code
// Use Error Handling to manage invalid inputs
// Use File I/O to save and load tasks from a file
// Use Command Line Arguments to allow users to specify actions when starting the app
// Use Iterators to traverse and manipulate the list of tasks
// Use Generics to allow for different types of task identifiers
// Use Lifetimes to manage references within the task list
// Use Closures for filtering and sorting tasks
// Use Smart Pointers for advanced memory management if needed
// Use Concurrency to allow multiple operations on the todo list simultaneously
// Use Testing to ensure the functionality of the todo app
// Use Documentation to explain the code and its usage
// Use Version Control to manage changes to the codebase
// Use Continuous Integration to automate testing and deployment
// Use Code Formatting to maintain a consistent style
// Use Code Linting to catch potential issues
// Use Code Reviews to ensure code quality
// Use Debugging to identify and fix issues
// Use Profiling to optimize performance
// Use Benchmarking to measure performance improvements
// Use Refactoring to improve code structure and readability
// Use Design Patterns to solve common problems in a reusable way


use std::io;

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

pub fn run() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n--- TODO MENU ---");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Mark Task Completed");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => remove_task(&mut tasks),
            "4" => mark_complete(&mut tasks),
            "5" => break,
            _ => println!("Invalid option"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    tasks.push(Task {
        title: title.trim().to_string(),
        completed: false,
    });

    println!("Task added!");
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("\n--- TASK LIST ---");
    for (i, task) in tasks.iter().enumerate() {
        println!(
            "{}. {} [{}]",
            i + 1,
            task.title,
            if task.completed { "DONE" } else { "PENDING" }
        );
    }
}

fn remove_task(tasks: &mut Vec<Task>) {
    println!("Enter task number to remove:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks.remove(index - 1);
            println!("Task removed!");
        } else {
            println!("Invalid index");
        }
    }
}

fn mark_complete(tasks: &mut Vec<Task>) {
    println!("Enter task number to mark complete:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].completed = true;
            println!("Task marked as completed!");
        } else {
            println!("Invalid index");
        }
    }
}
