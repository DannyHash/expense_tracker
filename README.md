# Expense Tracker CLI

A cool command-line application built in Rust to help me manage my personal finances. The Expense Tracker allows me to:

- **Add expenses** with amounts, categories, and timestamps.
- **View and sort expenses** by amount, category, or date.
- **Filter expenses** by category.
- **View monthly summaries** to track your spending habits.
- **Set budget limits** and receive alerts if you exceed them.
- **Delete expenses** if you make a mistake.
- **Export your data to CSV** for further analysis.
- Enjoy an **interactive, colorized CLI experience**.

This project is an excellent way to learn Rust fundamentals—ownership, error handling, pattern matching, data persistence, and more—while building a practical tool.

## Features

- **Add Expense**: Log an expense with its amount, category, and timestamp.
- **View Expenses**: Display a list of all recorded expenses with formatted output.
- **Sort Expenses**: Order your expenses by amount, category, or date.
- **Filter Expenses**: Narrow down expenses by category.
- **Monthly Summary**: Get a breakdown of your expenses for the current month.
- **Budget Limits & Alerts**: Set spending limits per category and get notified when you exceed them.
- **Delete Expense**: Remove an unwanted expense.
- **Persistent Data Storage**: Automatically save and load expenses from a file.
- **CSV Export**: Easily export your expenses to a CSV file for external use.
- **Enhanced CLI**: Utilize interactive menus and colored output for a smooth user experience.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (with Cargo)

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/expense-tracker-cli.git
   cd expense-tracker-cli
2. **Build the project:**
   ```bash
   cargo build --release

## Usage

- To run the Expense Tracker, simply execute:
  ```bash
  cargo run

You'll be greeted by an interactive menu where you can choose to add an expense, view your expense list, sort or filter your expenses, see a monthly summary, set a budget limit, delete an expense, or export your data to CSV. Follow the on-screen prompts to navigate through the options

## License

This project is licensed under the MIT License.
