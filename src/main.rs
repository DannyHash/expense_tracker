use chrono::{DateTime, Datelike, Utc};
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::result::Result;

// Define a struct to represent an exepense
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Expense {
    amount: f64,
    category: String,
    timestamp: DateTime<Utc>,
}

fn main() {
    // Print a welcome message
    println!("💰 Welcome to the Rust Expense Tracker!");

    // List of predefined categories
    let categories = ["Food", "Transport", "Entertainment", "Shopping", "Other"];

    // Create a vector to store expenses
    let mut expenses: Vec<Expense> = load_expenses();

    // Start an infiite loop to keep the program running
    loop {
        println!("\nMenu:");
        println!("1️⃣ Add expense");
        println!("2️⃣ View expenses");
        println!("3️⃣ Sort expenses");
        println!("4️⃣ Filter expenses");
        println!("5️⃣ Monthly Summary");
        println!("6️⃣ Delete an Expense");
        println!("7️⃣ Save & Exit");
        println!("8️⃣ Export to CSV");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read user input");
        let choice = choice.trim();

        match choice {
            "1" => add_expense(&mut expenses, &categories),
            "2" => view_expenses(&expenses),
            "3" => sort_expenses(&mut expenses),
            "4" => filter_expenses(&expenses),
            "5" => monthly_summary(&expenses),
            "6" => delete_expenses(&mut expenses),
            "7" => {
                save_expenses(&expenses);
                println!("👋 Exiting program... Goodbye!");
                break;
            }
            "8" => {
                if let Err(e) = export_to_csv(&expenses) {
                    println!("⚠️ Failed to export: {}", e);
                }
            }
            _ => println!("⚠️ Invalid choice! Please try again."),
        }
    }
}

// Function to add an expense
fn add_expense(expenses: &mut Vec<Expense>, categories: &[&str]) {
    // Prompt the User for input
    println!("\nEnter an expense amount:");

    // Create a mutable string to store user input
    let mut input = String::new();

    // Read user input from standard input (keyboard)
    io::stdin()
        .read_line(&mut input) // Read input and store in 'input'
        .expect("Failed to read user input"); // Handle potential errors

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
        .expect("Failed to read user input");
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
        timestamp: Utc::now(),
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
        println!(
            "Category: {}, Amount: ${:.2}, Date: {}",
            expense.category,
            expense.amount,
            expense.timestamp.format("%Y-%m-%d %H:%M:%S")
        );
    }

    println!("-------------------------");
}

// Function to sort expenses
fn sort_expenses(expenses: &mut Vec<Expense>) {
    println!("\n📌 Choose sorting option:");
    println!("1️⃣ By Amount (Low to High)");
    println!("2️⃣ By Amount (High to Low)");
    println!("3️⃣ By Category (A-Z)");
    println!("4️⃣ By Date (Newest First)");
    println!("5️⃣ By Date (Oldest First)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    let input = input.trim();

    match input {
        "1" => expenses.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap()),
        "2" => expenses.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap()),
        "3" => expenses.sort_by(|a, b| a.category.cmp(&b.category)),
        "4" => expenses.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)),
        "5" => expenses.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)),
        _ => {
            println!(" ⚠️ Invalid choice! Returning to menu");
            return;
        }
    }

    println!("\n✅ Expenses sorted!");
    view_expenses(expenses);
}

// Function to filter expenses
fn filter_expenses(expenses: &Vec<Expense>) {
    println!("\n📌 Enter category to filter:");

    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read user input");
    let category = category.trim();

    let filtered: Vec<&Expense> = expenses
        .iter()
        .filter(|expense| expense.category.eq_ignore_ascii_case(category))
        .collect();

    if filtered.is_empty() {
        println!("\n ⚠️ No expenses found for category: {}", category);
    } else {
        println!("\n📌 Expenses in category '{}':", category);
        println!("-------------------------");

        for expense in filtered {
            println!("Amount: ${:.2}", expense.amount);
        }
        println!("-------------------------");
    }
}

// Function to save expenses to a file
fn save_expenses(expenses: &Vec<Expense>) {
    let json = serde_json::to_string_pretty(expenses).expect("Failed to serialize expenses");
    let mut file = File::create("expenses.json").expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
    println!("💾 Expenses saved successfully!");
}

// Function to load expenses from a file
fn load_expenses() -> Vec<Expense> {
    match fs::read_to_string("expenses.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("⚠️ Error parsing file. Starting fresh.");
            Vec::new()
        }),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!("📂 No previous expenses found. Starting fresh.");
            Vec::new()
        }
        Err(_) => {
            println!("⚠️ Error reading file. Starting fresh.");
            Vec::new()
        } // Return empty list if file does not exist
    }
}

fn monthly_summary(expenses: &Vec<Expense>) {
    let now = Utc::now();
    let current_month = now.month();
    let current_year = now.year();

    let mut category_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    let mut total_spent = 0.0;

    for expense in expenses {
        if expense.timestamp.month() == current_month && expense.timestamp.year() == current_year {
            *category_totals
                .entry(expense.category.clone())
                .or_insert(0.0) += expense.amount;
            total_spent += expense.amount;
        }
    }

    if category_totals.is_empty() {
        println!("\n📂 No expenses recorded for this month.");
        return;
    }

    println!(
        "\n📊 Monthly Summary for {}/{}:",
        current_month, current_year
    );
    println!("-------------------------------------");

    for (category, total) in &category_totals {
        println!("Category: {}, Total Spent: ${:.2}", category, total);
    }

    println!("-------------------------------------");
    println!("💰 Total Spending This Month: ${:.2}", total_spent);
}

fn delete_expenses(expenses: &mut Vec<Expense>) {
    if expenses.is_empty() {
        println!("\n❌ No expenses to delete!");
        return;
    }

    println!("\n🗑️ Delete an Expense:");
    view_expenses(expenses);

    println!("\nEnter the index of the expense to delete:");

    let mut index_str = String::new();
    io::stdin()
        .read_line(&mut index_str)
        .expect("Failed to read user input");
    let index: usize = match index_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("⚠️ Invalid input! Please enter a valid index.");
            return;
        }
    };

    if index < expenses.len() {
        expenses.remove(index - 1);
        println!("✅ Expense deleted successfully!");
    } else {
        println!("⚠️ Invalid index! No expense deleted.");
    }
}

fn export_to_csv(expenses: &Vec<Expense>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(File::create("expense_csv")?);

    // Write CSV headers
    wtr.write_record(&["Category", "Amount", "Timestamp"])?;

    // Write each expense as a row
    for expense in expenses {
        wtr.write_record(&[
            &expense.category,
            &expense.amount.to_string(),
            &expense.timestamp.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("📁 Expenses exported to `expenses.csv` successfully!");
    Ok(())
}
