use crate::finance_tracker::FinanceTracker;

pub fn display_expense_chart(tracker: &FinanceTracker) {
    let breakdown = tracker.expense_breakdown();

    if breakdown.is_empty() {
        println!("No expenses to list.");
        return;
    }

    let total = tracker.total_expenses();
    let max_bar_length = 50;

    println!("\nExpense Distribution:");
    println!("=====================");

    let mut categories: Vec<_> = breakdown.iter().collect();
    categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (category, amount) in categories {
        let percentage = (amount / total) * 100.0;
        let bar_length = ((amount / total) * max_bar_length as f64) as usize;
        let bar = "█".repeat(bar_length);
        
        println!(
            "{:<15} {:>6.1}% {} ${:.2}",
            category, percentage, bar, amount
        );
    }
    
    println!("\nTotal: ${:.2}", total);
}