use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

// Define a struct to represent an exepense
#[derive(Serialize, Deserialize)]
struct Expense {
    amount: f64,
    category: String,
}

const FILE_PATH: &str = "expense.json";

fn main() {
    // Print a welcome message
    println!("💰 Welcome to the Rust Expense Tracker!");
    println!("Type 'exit' to exit the program");

    // List of predefined categories
    let categories = ["Food", "Transport", "Entertainment", "Shopping", "Other"];

    // Create a vector to store expenses
    let mut expenses: Vec<Expense> = load_expenses();

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
        let amount: f64 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("⚠️ Invalid input! Please enter a valid number");
                continue;
            }
        };

        // Ask user to select a category
        println!("\nSelect a category");
        for (i, category) in categories.iter().enumerate() {
            println!("{}: {}", i + 1, category);
        }

        let mut category_input = String::new();
        io::stdin()
            .read_line(&mut category_input)
            .expect("Failed to read input");
        let category_input = category_input.trim();

        // Convert category input to an index
        let category_index: usize;
        if let Ok(num) = category_input.parse::<usize>() {
            if num > 0 && num <= categories.len() {
                category_index = num - 1;
            } else {
                println!("⚠️ Invalid choice! Using 'Other' as default.");
                category_index = categories.len() - 1;
            }
        } else {
            println!("⚠️ Invalid choice! Using 'Other' as default.");
            category_index = categories.len() - 1;
        }

        // Create an expense instance and store it
        let expense = Expense {
            amount,
            category: categories[category_index].to_string(),
        };
        expenses.push(expense);

        // Display all recorded expenses
        println!("\n📔 Your expenses:");
        for exp in &expenses {
            println!("- ${:.2} ({})", exp.amount, exp.category);
        }
    }

    save_expenses(&expenses);
    println!("\n✅ Expenses saved");

    // After exiting, show summary stats
    display_summary(&expenses);
}

// Function to save expenses to a file
fn save_expenses(expenses: &Vec<Expense>) {
    let json = serde_json::to_string_pretty(expenses).expect("Failed to serialize expenses");
    fs::write(FILE_PATH, json).expect("Failed to write to file");
}

// Function to load expenses from a file
fn load_expenses() -> Vec<Expense> {
    match fs::read_to_string(FILE_PATH) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(), // Return empty list if file does not exist
    }
}

// Function to display summary stats
fn display_summary(expenses: &Vec<Expense>) {
    let mut total: f64 = 0.0;
    let mut category_totals: HashMap<String, f64> = HashMap::new();

    // Calculate total expenses and category totals
    for expense in expenses {
        total += expense.amount;
        let counter = category_totals.entry(expense.category.clone()).or_insert(0.0);
        *counter += expense.amount;
    }

    println!("\n💰 Expense summary");
    println!("--------------------------------");
    println!("Total spent: ${:.2}", total);
    println!("--------------------------------");

    for (category, amount) in &category_totals {
        println!("{}: ${:.2}", category, amount);
    }
    println!("--------------------------------");
}