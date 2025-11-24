use crate::transaction::{Transaction, TransactionType};
use std::collections::HashMap;

// strucure
pub struct FinanceTracker {
    transactions: Vec<Transaction>,
    next_id: u32,
}

// implementation
impl FinanceTracker {
    pub fn new() -> Self {
        FinanceTracker {
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_income(&mut self, title: String, amount: f64, description: Option<String>) {
        let transaction = Transaction::new_income(self.next_id, title, amount, description);
        self.transactions.push(transaction);
        self.next_id += 1;
    }

    pub fn add_expense(
        &mut self,
        title: String,
        amount: f64,
        category: String,
        description: Option<String>,
    ) {
        let transaction = Transaction::new_expense(self.next_id, title, amount, category, description);
        self.transactions.push(transaction);
        self.next_id += 1;
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn display_all(&self) {
        if self.transactions.is_empty() {
            println!("No transactions yet.");
            return;
        }

        for transaction in &self.transactions {
            println!("{}", transaction.display());
            println!();
        }
    }

    pub fn expense_breakdown(&self) -> HashMap<String, f64> {
        let mut breakdown = HashMap::new();

        for transaction in &self.transactions {
            if transaction.is_expense() {
                if let Some(category) = &transaction.category {
                    *breakdown.entry(category.clone()).or_insert(0.0) += transaction.amount
                }
            }
        }
        
        breakdown
    }
    
    pub fn total_expenses(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.is_expense())
            .map(|t| t.amount)
            .sum()
    }
    
    pub fn total_income(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.is_income())
            .map(|t| t.amount)
            .sum()
    }
    
    pub fn net_balance(&self) -> f64 {
        self.total_income() - self.total_expenses()
    }
}