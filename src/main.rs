use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

// Define a struct to represent an exepense
#[derive(Serialize, Deserialize, Debug)]
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
        println!("\nMenu:");
        println!("1️⃣ Add expense");
        println!("2️⃣ View expenses");
        println!("3️⃣ Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => add_expense(&mut expenses, &categories),
            "2" => view_expenses(&expenses),
            "3" => {
                println!("👋 Exiting... Goodbye!");
                break;
            },
            _ => println!("Invalid choice! Please enter 1, 2 or 3"),
        }
    }
}

// Function to add an expense
fn add_expense(expenses: &mut Vec<Expense>, categories: &[&str]) {
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
    
            // Convert the input to a floating point number
            let amount: f64 = match input.parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("⚠️ Invalid input! Please enter a valid number");
                    return;
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

            save_expenses(&expenses);
            println!("\n✅ Expenses saved");
}

// Function to view all recorded expenses
fn view_expenses(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("📂 No expenses were recorded");
        return;
    }

    println!("\n💰 Your Expenses:");
    println!("-------------------------");

    for expense in expenses {
        println!("Category: {}, Amount: ${:.2}", expense.category, expense.amount);
    }

    println!("-------------------------");
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