#![deny(missing_docs)]
#![deny(rustdoc::private_doc_tests)]
//! Rust-based financial management service

mod data_structures;
pub use crate::data_structures::{Date, Transaction, Id};

/// Enum defining error types for TreasuryService
pub enum TreasuryServiceError {}

/// Main encapsulating struct
pub struct TreasuryService {}

impl TreasuryService {
    /// Get a `Result<Vec<Transaction>, TreasuryServiceError>` of all transactions between 2 given dates,
    /// may return an `Err` member if query fails.
    ///
    /// Query may fail if:
    /// - `from` date occurs after `to` date
    pub fn get_transactions(&self, _from: Date, _to: Date) -> Result<Vec<Transaction>, TreasuryServiceError> {
        todo!();
    }

    /// Add a transaction to the system, may return `Err` memeber if operation fails.
    pub fn add_transaction(&mut self, _transaction: Transaction) -> Result<(), TreasuryServiceError> {
        todo!();
    }
    
    /// Overwrite data of a transaction with a specified `transaction_id`, may return `Err` member
    /// if operation fails.
    ///
    /// Operation may fail if:
    /// - Therre is no transaction with the given `transaction_id`
    pub fn set_transaction(&mut self, _transaction_id: Id, _transaction: Transaction) -> Result<(), TreasuryServiceError> {
        todo!();
    }

    /// Remove a transaction from the system with a given `transaction_id`, may return `Err` member if operation fails.
    ///
    /// Operation may fail if:
    /// - There is no transaction with the given `transaction_id`
    pub fn remove_transaction(&mut self, _transaction_id: Id) -> Result<(), TreasuryServiceError> {
        todo!();
    }
}
