use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{Date, Id, Money};

/// Struct to encapsulate a financial transaction
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: Id,
    amount: Money,
    date: Date,
}

impl Transaction {
    /// Create a new transaction with the given id, amount and datetime
    pub fn new(id: Id, amount: Money, date: Date) -> Self {
        Self { id, amount, date }
    }

    /// Get Id for transaction
    pub fn get_id(&self) -> Id {
        self.id
    }

    /// Get amount in transaction
    pub fn get_amount(&self) -> Money {
        self.amount
    }

    /// Get date of the transaction
    pub fn get_date(&self) -> &Date {
        &self.date
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{} {}: £{}.{:0>2}",
            self.id,
            self.date,
            self.amount / 100,
            self.amount % 100
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt() {
        let t = Transaction {
            id: 0,
            amount: 105,
            date: Date::new(6, 7, 2004),
        };

        assert_eq!(t.to_string(), "#0 06-07-2004: £1.05")
    }
}
