use crate::finance_tracker::FinanceTracker;
use std::fs::File;
use std::io::{self, Write};

pub fn generate_report(tracker: &FinanceTracker, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "======================================")?;
    writeln!(file, "       Personal Finance Report")?;
    writeln!(file, "======================================")?;
    writeln!(file)?;

    writeln!(file, "SUMMARY:")?;
    writeln!(file, "Total income:   ${:.2}", tracker.total_income())?;
    writeln!(file, "Total expenses: ${:.2}", tracker.total_expenses())?;
    writeln!(file, "Net Balance:    ${:.2}", tracker.net_balance())?;
    writeln!(file)?;

    writeln!(file, "Expense Breakdown By Category:")?;
    writeln!(file, "--------------------------------------")?;
    let breakdown = tracker.expense_breakdown();

    if breakdown.is_empty() {
        writeln!(file, "No expenses recorded.")?;
    } else {
        let mut categories: Vec<_> = breakdown.iter().collect();
        categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        for (category, total) in categories {
            let percentage = (total / tracker.total_expenses()) * 100.0;
            writeln!(file, "{:<20} ${:>10.2} ({:.1}%)", category, total, percentage)?;
        }
    }

    writeln!(file)?;
    writeln!(file, "ALL TRANSACTIONS:")?;
    writeln!(file, "--------------------------------------")?;

    for transaction in tracker.transactions() {
        writeln!(file, "{}", transaction.display())?;
        writeln!(file)?;
    }

    Ok(())
}