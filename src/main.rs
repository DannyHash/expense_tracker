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

/*
Expense Struct:
- amount (f64): The expense value for arithmetic ops (e.g., total += expense.amount).
- category (String): Expense type for control-flow (e.g., if expense.category == "Food").
- timestamp (DateTime<Utc>): When the expense occurred, for sorting/filtering by date.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Expense {
    amount: f64,
    category: String,
    timestamp: DateTime<Utc>,
}

/*
   ExpenseTracker Struct:
   - expenses (Vec<Expense>): A collection of expense entries for arithmetic operations (e.g., summing totals).
   - budgets (HashMap<String, f64>): Budget limits by category, used in control-flow for budget checks.
*/
struct ExpenseTracker {
    expenses: Vec<Expense>,
    budgets: HashMap<String, f64>, // Stores budget limits per category
}

/*
   ExpenseTracker Implementation:
   - new() -> Self: Constructs a new, empty ExpenseTracker.
   - Initializes:
       • expenses with Vec::new() for collecting expense entries.
       • budgets with HashMap::new() for storing category budget limits.
*/
impl ExpenseTracker {
    fn new() -> Self {
        Self {
            expenses: Vec::new(),
            budgets: HashMap::new(),
        }
    }
}

fn main() {
    println!("💰 Welcome to the Rust Expense Tracker!");

    let mut tracker = ExpenseTracker::new();
    tracker.expenses = load_expenses();

    /*
       Main Loop:
       - Defines menu choices (with emojis) for various expense tracker actions.
       - Uses an interactive prompt (via Select) to capture the user's selection.
       - Executes the corresponding function based on the choice
    */
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

        /*
           This match block controls the program's flow based on the user's menu selection:
           - 0: Call add_expense, passing a mutable reference to the tracker.
           - 1: Call view_expenses, displaying the list of expenses.
           - 2: Call sort_expenses to order the expenses.
           - 3: Call filter_expenses to show a subset of expenses.
           - 4: Call monthly_summary to generate a report.
           - 5: Call set_budget to adjust budget limits.
           - 6: Call delete_expenses to remove an expense.
           - 7: Attempt to export expenses to CSV; if it fails, print an error message.
           - 8: Save expenses, print a goodbye message, and break out of the loop to exit.
           - _: Handle any invalid selection with a warning message.
        */
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

    /*
       Adds a new expense entry to the tracker's expenses vector:
       - category: Clones the category string to ensure ownership.
       - amount: Uses the provided expense value (f64) for calculations.
       - timestamp: Records the current UTC time using chrono::Utc::now().
    */
    tracker.expenses.push(Expense {
        category: category.clone(),
        amount,
        timestamp: chrono::Utc::now(),
    });

    println!("✅ Expense added: {} - ${:.2}", category, amount);

    /*
       Checks if a budget exists for the given category and warns if spending exceeds it.

       - `if let Some(&budget) = tracker.budgets.get(&category)`:
           Attempts to retrieve the budget for the category.
           If found, destructures the value (using & to dereference) into `budget`.

       - Calculates total spending for the category:
           • Iterates over `tracker.expenses`.
           • Filters expenses that match the category.
           • Maps each expense to its amount.
           • Sums all amounts to get `total_spent` (arithmetic sum of f64 values).

       - Compares `total_spent` with the budget:
           If spending exceeds the budget, prints a warning message.
    */
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

/*
   view_expenses Function:
   - Displays the list of recorded expenses in a formatted manner.
   - Steps:
       1. Prints a header ("Expense List") with bold and underline formatting.
       2. Checks if there are any expenses:
            • If empty, prints a warning and exits the function.
       3. Otherwise, prints a sub-header ("Your Expenses") and a divider.
       4. Iterates through expenses with enumeration:
            • Formats and prints each expense with its index, category, timestamp, and amount.
       5. Ends by printing a closing divider.
*/
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

    /*
       Match on the user input to sort the expenses vector accordingly:

       - "1": Sort expenses in ascending order by amount.
              Uses partial_cmp to compare f64 values (unwrap assumes no NaN).
       - "2": Sort expenses in descending order by amount.
              Reverses the order by swapping a and b.
       - "3": Sort expenses alphabetically by category.
              Uses cmp for String comparison.
       - "4": Sort expenses in descending order by timestamp.
              The most recent expenses come first.
       - "5": Sort expenses in ascending order by timestamp.
              The oldest expenses come first.
       - _ (any other input): Print an error message and return from the function.
    */
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

/*
   filter_expenses function:
   - Prompts the user to enter a category for filtering.
   - Reads and trims user input from stdin.
   - Filters the expenses vector, selecting only those expenses whose category
     matches the input, ignoring case differences.
   - If no matching expenses are found, prints a warning.
   - Otherwise, prints the amounts for all matching expenses.
*/
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

/*
   save_expenses function:
   - Serializes the 'expenses' vector into a pretty-formatted JSON string using serde_json.
   - Creates (or overwrites) a file named "expenses.json" for storing the data.
   - Writes the JSON string to the file as bytes.
   - Prints a confirmation message upon successful saving.
*/
fn save_expenses(expenses: &Vec<Expense>) {
    let json = serde_json::to_string_pretty(expenses).expect("Failed to serialize expenses");
    let mut file = File::create("expenses.json").expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
    println!("💾 Expenses saved successfully!");
}

fn load_expenses() -> Vec<Expense> {
    /*
       Reads the "expenses.json" file and attempts to deserialize its contents into a vector of expenses.

       Control Flow:
       - If the file is read successfully (Ok(data)):
           • Attempts to parse the JSON data.
           • On parsing error, prints an error message and returns an empty vector.
       - If the file is not found (ErrorKind::NotFound):
           • Informs the user no previous expenses were found and returns an empty vector.
       - For any other file read error:
           • Prints a general error message and returns an empty vector.
    */
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

    /*
       For each expense in the expenses list:
       - Checks if the expense's timestamp matches the current month and year.
       - If it matches:
           • Updates category_totals:
               - Uses .entry() with a cloned category string.
               - Inserts 0.0 if the category is not present.
               - Adds the expense amount to the existing total.
           • Adds the expense amount to total_spent.
    */
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

/*
   set_budget Function:
   - Prompts the user to enter a category to set a budget for.
   - Prompts the user to input the budget limit for that category.
   - Inserts the category and its budget into the tracker’s budgets (a HashMap).
   - Prints a confirmation message showing the budget set.
*/
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

fn export_to_csv(expenses: &Vec<Expense>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(File::create("expense_csv")?);

    // Write CSV headers
    wtr.write_record(&["Category", "Amount", "Timestamp"])?;

    /*
       Iterates over each expense in the expenses vector and writes its data as a CSV record:
       - expense.category: Directly written as the category string.
       - expense.amount.to_string(): Converts the amount (f64) to a string.
       - expense.timestamp.to_string(): Converts the timestamp to a string.
       The '?' operator propagates any errors that occur during writing.
    */
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

/*
   delete_expenses Function:
   - Checks if the expenses list is empty; if so, prints a message and exits.
   - Prompts the user to enter the index of an expense to delete.
   - Displays the current list of expenses using view_expenses.
   - Reads user input as a string and attempts to parse it into a usize index.
   - If parsing fails, prints an error and returns.
   - Adjusts for 1-based user input by removing the expense at (index - 1) if the index is valid.
   - Prints a success message on deletion, or an error if the index is out of range.
*/
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
