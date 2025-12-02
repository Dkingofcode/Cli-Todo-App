use std::io::{self, Write};

pub fn run() {
    println!("=====================================");
    println!("     🚀 Rust Interactive Calculator");
    println!("=====================================\n");

    loop {
        let first_number = read_number("Enter first number: ");
        let operator = read_operator("Enter operator (+, -, *, /): ");
        let second_number = read_number("Enter second number: ");

        let result = operate(operator, first_number, second_number);

        println!("\n➡  {} {} {} = \x1b[32m{}\x1b[0m\n",
            first_number, operator, second_number, result);

        if !ask_yes_no("Do you want to calculate again? (y/n): ") {
            println!("\nThanks for using the calculator! 👋");
            break;
        }

        println!("\n-------------------------------------\n");
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<f64>() {
            return num;
        } else {
            println!("\x1b[31mInvalid number. Try again.\x1b[0m");
        }
    }
}

fn read_operator(prompt: &str) -> char {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();

        if let Some(c) = op.trim().chars().next() {
            if "+-*/".contains(c) {
                return c;
            }
        }
        println!("\x1b[31mInvalid operator. Try again.\x1b[0m");
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();

        match ans.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("\x1b[31mPlease enter 'y' or 'n'.\x1b[0m"),
        }
    }
}

fn operate(op: char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => unreachable!("Operator already validated"),
    }
}
