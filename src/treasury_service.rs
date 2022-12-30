use crate::{
    data_structures::{Id, Money, Transaction},
    database_driver::{DatabaseDriver, DatabaseDriverError},
};
use chrono::NaiveDateTime;
use thiserror::Error;

/// Enum defining error types for TreasuryService
#[derive(Debug, Error)]
pub enum TreasuryServiceError {
    #[error("Database error: {0}")]
    /// Variant for errors occuring in the underlying database
    DatabaseError(#[from] DatabaseDriverError),
}

/// Main encapsulating struct
#[derive(Debug)]
pub struct TreasuryService {
    database_driver: DatabaseDriver,
}

impl TreasuryService {
    /// Create new service
    pub async fn new(path: &str) -> Result<TreasuryService, TreasuryServiceError> {
        let database_driver = DatabaseDriver::new(path).await?;
        Ok(TreasuryService { database_driver })
    }

    /// Get a `Result<Vec<Transaction>, TreasuryServiceError>` of all transactions in the database
    pub async fn get_transactions(&mut self) -> Result<Vec<Transaction>, TreasuryServiceError> {
        Ok(self.database_driver.get_transactions().await?)
    }

    /// Add a transaction to the system, may return `Err` memeber if operation fails.
    pub async fn add_transaction(
        &mut self,
        amount: Money,
        datetime: NaiveDateTime,
    ) -> Result<(), TreasuryServiceError> {
        let id = self.database_driver.get_next_id().await?;
        let new_transaction = Transaction::new(id, amount, datetime);

        Ok(self
            .database_driver
            .add_transaction(new_transaction)
            .await?)
    }

    /// Overwrite data of a transaction with a specified `transaction_id`, may return `Err` member
    /// if operation fails.
    ///
    /// Operation may fail if:
    /// - There is no transaction with the given `transaction_id`
    pub async fn set_transaction(
        &mut self,
        id: Id,
        amount: Money,
        datetime: NaiveDateTime,
    ) -> Result<(), TreasuryServiceError> {
        let updated_transaction = Transaction::new(id, amount, datetime);

        Ok(self
            .database_driver
            .set_transaction(updated_transaction)
            .await?)
    }

    /// Remove a transaction from the system with a given `transaction_id`, may return `Err` member if operation fails.
    ///
    /// Operation may fail if:
    /// - There is no transaction with the given `transaction_id`
    pub async fn remove_transaction(&mut self, id: Id) -> Result<(), TreasuryServiceError> {
        Ok(self.database_driver.remove_transaction(id).await?)
    }
}
