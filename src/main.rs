// Importing the input/output libary from Rust's standard library
use std::io;

fn main() {
    // Print a welcome message
    println!("💰 Welcome to the Rust Expense Tracker!");
    println!("Type 'exit' to exit the program");

    // Create a vector to store expenses
    let mut expenses: Vec<f64> = Vec::new();

    // Start an infiite loop to keep the program running
    loop {
        // Prompt the User for input
        println!("\nEnter an expense amount(or 'exit' to exit the program):");

        // Create a mutable string to store user input
        let mut input = String::new();

        // Read user input from standard input (keyboard)
        io::stdin()
            .read_line(&mut input) // Read input and store in 'input'
            .expect("Failed to read user input from standard input"); // Handle potential errors

        // Trim any leading or trailing whitespace from the input and check if the user wants to exit
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("👋 Exiting Expense Tracker. Goodbye!");
            break; // Exit the loop
        }

        // Convert the input to a floating point number
        match input.parse::<f64>() {
            Ok(amount) => {
                expenses.push(amount); // Store the expense
                println!("✅ Added Expense: ${:.2}", amount);
            }
            Err(_) => {
                println!("⚠️ Invalid input! Please enter a valid number");
            }
        }

        // Show all recorded expenses
        println!("📔 Your expenses: {:?}", expenses);
    }
}