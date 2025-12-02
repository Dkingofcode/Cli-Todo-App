# 📝 CLI Todo App + Calculator Module

A Rust-powered command-line application that combines **a Todo List Manager**, **a Basic Calculator**, and **a Multithreaded Calculator** — all in one tool.

This project is designed to help beginners and intermediates strengthen their understanding of **Rust fundamentals**, including ownership, borrowing, modules, collections, user input, and multithreading.

---

## 🚀 Features

### ✅ **Todo App**
- Add tasks  
- List tasks  
- Remove tasks  
- Mark tasks as completed  
- Clean, simple CLI interface  

### 🧮 **Calculator Module**
- Addition  
- Subtraction  
- Multiplication  
- Division (with safe error handling)  
- Separated into its own Rust module for reusability  

### ⚡ **Multithreaded Calculator**
- Executes operations on **multiple threads**  
- Demonstrates concurrency using `std::thread`  
- Ideal for CPU-heavy or background computations  

---

## 🧩 Project Structure

Each module handles a dedicated portion of the program, designed using clean Rust architecture.

---

## 📦 Installation

First, ensure Rust is installed:


rustc --version
cargo --version



## ▶️ Running the App

Inside the project directory, run:

cargo run


## Welcome to the CLI Todo App with Calculator Module!
Select an option:
1) Todo App
2) Calculator
3) Multithreaded Calculator
4) Exit


## 📘 Usage Guide
📝 Todo App Menu

Selecting Todo App gives you:

1) Add Task
2) List Tasks
3) Remove Task
4) Mark Task as Completed
5) Back to Main Menu

Examples

Add → "Finish Rust project"

List → Shows pending & completed tasks

Remove → Remove by index

Mark complete → Updates task status

## 🧮 Calculator Usage

Input order:

Enter first number
Enter second number
Choose operation: +, -, *, /

Example
5
3
*
Result: 15

## ⚡ Multithreaded Calculator Usage

You will see:

Enter number A:
Enter number B:
Choose operation to run in a separate thread:


The calculation runs on a separate thread, demonstrating Rust concurrency.

## 🧠 Concepts Demonstrated

This project reinforces many core Rust concepts:

Rust module organization

Structs & enums

Ownership and borrowing

Collections (Vec<Task>)

Pattern matching

Error handling

Iterators

User input parsing

Concurrency with threads

A strong foundation for advanced Rust and future Solana development.

## 🛠️ Optional Future Enhancements

Ideas to extend the app:

File-based task persistence

Task timestamps

Task categories or priority levels

Colored output

Batch operations using threads

Terminal UI using crossterm or ratatui

📄 License

This project is open-source.
Feel free to modify, extend, or use it for learning.
