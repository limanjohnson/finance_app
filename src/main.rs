mod transaction;
mod finance_tracker;
mod report;
mod visualization;

use finance_tracker::FinanceTracker;
use std::io::{self, Write};

fn main() {
    let mut tracker = FinanceTracker::new();

    loop{
        println!("\n=== Your Finance Tracker ===");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View All Transacrions");
        println!("4. View Expense Breakdown");
        println!("5. Generate Report");
        println!("6. Exit");
        print!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_income(&mut tracker),
            "2" => add_expense(&mut tracker),
            "3" => view_transactions(&tracker),
            "4" => view_breakdown(&tracker),
            "5" => generate_report(&tracker),
            "6" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn add_income(tracker: &mut FinanceTracker) {
    println!("\n--- Add Income ---");

    let title = get_input("Title: ");
    let amount = get_amount();
    let description = get_optional_input("Description (optional): ");

    tracker.add_income(title, amount, description);
    println!("Income added successfully!");
}

fn add_expense(tracker: &mut FinanceTracker) {
    println!("\n--- Add Expense ---");

    let title = get_input("Title: ");
    let amount = get_amount();
    let category = get_input("Category: ");
    let description = get_optional_input("Description (optional): ");

    tracker.add_expense(title, amount, category, description);
    println!("Expense added successfully!");
}

fn view_transactions(tracker: &FinanceTracker) {
    println!("\n--- All Transactions ---");
    tracker.display_all();
}

fn view_breakdown(tracker: &FinanceTracker) {
    println!("\n--- Expense Breakdown by Category ---");
    visualization::display_expense_chart(tracker);

    println!("\nTotal Income: ${:.2}", tracker.total_income());
    println!("Net Balance: ${:.2}", tracker.net_balance());
}

fn generate_report(tracker: &FinanceTracker) {
    println!("\n--- Generate Report ---");
    print!("Enter filename (e.g., report.txt): ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match report::generate_report(tracker, filename) {
        Ok(_) => println!("✓ Report saved to {}", filename),
        Err(e) => println!("✗ Error generating report: {}", e),
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_optional_input(prompt: &str) -> Option<String> {
    let input = get_input(prompt);
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn get_amount() -> f64 {
    loop {
        print!("Amount: $");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(amount) if amount > 0.0 => return amount,
            _ => println!("Please enter a valid positive number."),
        }
    }
}