use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug, Clone)]
// Structure
pub struct Transaction {
    pub id: u32,
    pub title: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub category: Option<String>,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
}

// Implementation
impl Transaction {
    pub fn new_income(
        id: u32,
        title: String,
        amount: f64,
        description: Option<String>,
    ) -> Self {
        Transaction {
            id,
            title,
            amount,
            transaction_type: TransactionType::Income,
            category: None,
            description,
            date: Utc::now(),
        }
    }

    pub fn new_expense(
        id: u32,
        title: String,
        amount: f64,
        category: String,
        description: Option<String>,
    ) -> Self {
        Transaction {
            id,
            title,
            amount,
            transaction_type: TransactionType::Expense,
            category: Some(category),
            description,
            date: Utc::now(),
        }
    }

    pub fn is_expense(&self) -> bool {
        matches!(self.transaction_type, TransactionType::Expense)
    }

    pub fn is_income(&self) -> bool {
        matches!(self.transaction_type, TransactionType::Income)
    }

    pub fn display(&self) -> String {
        let type_str = match self.transaction_type {
            TransactionType::Income => "INCOME",
            TransactionType::Expense => "EXPENSE",
        };

        let mut output = format!(
            "[{}] {} - ${:.2} ({})",
            type_str,
            self.title,
            self.amount,
            self.date.format("%Y-%m-%d %H:%M")
        );

        if let Some(category) = &self.category {
            output.push_str(&format!(" | Category: {}", category));
        }

        if let Some(description) = &self.description {
            output.push_str(&format!("\n    Description: {}", description));
        }

        output
    }
}