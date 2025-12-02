use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

pub fn run() {
    println!("--- Multithreaded Calculator ---");

    print!("Enter numbers separated by spaces: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    if nums.is_empty() {
        println!("No numbers.");
        return;
    }

    let result = Arc::new(Mutex::new(0.0));

    let threads: Vec<_> = nums.into_iter().map(|n| {
        let result_clone = Arc::clone(&result);

        thread::spawn(move || {
            let mut res = result_clone.lock().unwrap();
            *res += n * 2.0;
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("Threaded sum-doubled result: {}", *result.lock().unwrap());
}



