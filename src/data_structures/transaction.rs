use chrono::NaiveDateTime;
use std::fmt::Display;

use super::{Id, Money};

/// Struct to encapsulate a financial transaction
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    id: Id,
    amount: Money,
    datetime: NaiveDateTime,
}

impl Transaction {
    /// Create a new transaction with the given id, amount and datetime
    pub fn new(id: Id, amount: Money, datetime: NaiveDateTime) -> Self {
        Self {
            id,
            amount,
            datetime,
        }
    }

    /// Get Id for transaction
    pub fn get_id(&self) -> Id {
        self.id
    }

    /// Get amount in transaction
    pub fn get_amount(&self) -> Money {
        self.amount
    }

    /// Get datetime in transaction
    pub fn get_datetime(&self) -> NaiveDateTime {
        self.datetime
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{} {}: £{}.{:0>2}",
            self.id,
            self.datetime.format("%d-%m-%Y"),
            self.amount / 100,
            self.amount % 100
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn fmt() {
        let t = Transaction {
            id: 0,
            amount: 105,
            datetime: NaiveDateTime::parse_from_str("06-07-2004 00:00:00", "%d-%m-%Y %H:%M:%S")
                .unwrap(),
        };

        assert_eq!(t.to_string(), "#0 06-07-2004: £1.05")
    }
}
