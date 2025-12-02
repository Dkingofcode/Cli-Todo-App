mod todo;
mod calculator;
mod multithreaded_calc;

// Build a cli-todo APp that lists a set of todo items and allows the user to add, remove, and mark items as completed. But also i am implementing
// a calculator module that can perform basic arithmetic operations like addition, subtraction, multiplication, and division. And a Multithreaded calculator APP 
/// to top it al off. So User can choose between using a normal calculator, a multithreaded calculator or the todo app itself.


// Requirements: 
// A menu System
// Loops for user input
// Match on user selection
// Calls to the todo system and calculator modules


use std::io::{self, Write};




fn main() {

    loop{
        println!("\n=== MAIN MENU ===");
        println!("1. Todo App");
        println!("2. Calculator");
        println!("3. Multithreaded Calculator");
        println!("4. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => todo::run(),
            "2" => calculator::run(),
            "3" => multithreaded_calc::run(),
            "4" => {
                println!("Exiting the application. Goodbye!");
                break;
        }
        _ => println!("Invalid choice, please try again."),
    }    
    }
     //   println!("Welcome to the CLI Todo App with Calculator Module!");
    // Here you can add code to interact with the user, display the todo list, and
    // handle calculator operations.



}
