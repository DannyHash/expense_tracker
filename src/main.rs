use chrono::{DateTime, Datelike, Utc};
use colored::*;
use csv::Writer;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::result::Result;

// Define a struct to represent an exepense
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Expense {
    amount: f64,
    category: String,
    timestamp: DateTime<Utc>,
}

struct ExpenseTracker {
    expenses: Vec<Expense>,
    budgets: HashMap<String, f64>, // Stores budget limits per category
}

impl ExpenseTracker {
    fn new() -> Self {
        Self {
            expenses: Vec::new(),
            budgets: HashMap::new(),
        }
    }
}

fn main() {
    // Print a welcome message
    println!("💰 Welcome to the Rust Expense Tracker!");

    // List of predefined categories
    // let categories = ["Food", "Transport", "Entertainment", "Shopping", "Other"];

    // Create an instance of ExpenseTracker
    let mut tracker = ExpenseTracker::new();
    tracker.expenses = load_expenses();

    // Start an infiite loop to keep the program running
    loop {
        let choices = vec![
            "➕ Add Expense",
            "📋 View Expenses",
            "📊 Sort Expenses",
            "📊 Filter Expenses",
            "📅 Monthly Summary",
            "⚠️ Set Budget Limit",
            "🗑️ Delete an Expense",
            "📁 Export to CSV",
            "💾 Save & Exit",
        ];

        let selection = Select::new()
            .with_prompt("📌 Choose an option")
            .default(0)
            .items(&choices)
            .interact()
            .unwrap();

        match selection {
            0 => add_expense(&mut tracker),
            1 => view_expenses(&mut tracker.expenses),
            2 => sort_expenses(&mut tracker.expenses),
            3 => filter_expenses(&tracker.expenses),
            4 => monthly_summary(&tracker.expenses),
            5 => set_budget(&mut tracker),
            6 => delete_expenses(&mut tracker.expenses),
            7 => {
                if let Err(e) = export_to_csv(&tracker.expenses) {
                    println!("⚠️ Failed to export: {}", e);
                }
            }
            8 => {
                save_expenses(&tracker.expenses);
                println!("👋 Exiting program... Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid choice! Please try again."),
        }
    }
}

// Function to add an expense
fn add_expense(tracker: &mut ExpenseTracker) {
    let category: String = Input::new()
        .with_prompt("Enter expense category:")
        .interact_text()
        .unwrap();

    let amount: f64 = Input::new()
        .with_prompt("Enter expense amount:")
        .interact_text()
        .unwrap();

    tracker.expenses.push(Expense {
        category: category.clone(),
        amount,
        timestamp: chrono::Utc::now(),
    });

    println!("✅ Expense added: {} - ${:.2}", category, amount);

    // Check if budget is exceeded
    if let Some(&budget) = tracker.budgets.get(&category) {
        let total_spent: f64 = tracker
            .expenses
            .iter()
            .filter(|e| e.category == category)
            .map(|e| e.amount)
            .sum();

        if total_spent > budget {
            println!(
                "⚠️ Warning: You have exceeded your budget of ${:.2} for '{}'.",
                budget, category
            );
        }
    }
}

// Function to view all recorded expenses
fn view_expenses(expenses: &Vec<Expense>) {
    println!("\n{}", "📋 Expense List".bold().underline());

    if expenses.is_empty() {
        println!("{}", "⚠️ No expenses recorded yet.".yellow());
        return;
    }

    println!("\n💰 Your Expenses:");
    println!("-------------------------");

    for (i, expense) in expenses.iter().enumerate() {
        println!(
            "{} {} - {} - ${:.2}",
            format!("#{}", i + 1).cyan(),
            expense.category.green(),
            expense.timestamp.to_string().purple(),
            expense.amount
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

fn set_budget(tracker: &mut ExpenseTracker) {
    let category: String = Input::new()
        .with_prompt("Enter category name to set a budget for")
        .interact_text()
        .unwrap();

    let budget: f64 = Input::new()
        .with_prompt(format!("Enter budget limit for '{}'", category))
        .interact_text()
        .unwrap();

    tracker.budgets.insert(category.clone(), budget);
    println!(
        "✅ Budget of ${:.2} set for category '{}'",
        budget, category
    );
}

fn delete_expenses(expenses: &mut Vec<Expense>) {
    if expenses.is_empty() {
        println!("\n❌ No expenses to delete!");
        return;
    }

    println!("\n 🗑️ Delete an Expense:");
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
