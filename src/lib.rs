mod data_structures;
use crate::data_structures::{Date, Transaction, Id};

pub enum TreasuryServiceError {}

pub struct TreasuryService {}

impl TreasuryService {
    pub fn get_transactions(&self, from: Date, to: Date) -> Result<Vec<Transaction>, TreasuryServiceError> {
        todo!();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), TreasuryServiceError> {
        todo!();
    }
    
    pub fn set_transaction(&mut self, transaction_id: Id, transaction: Transaction) -> Result<(), TreasuryServiceError> {
        todo!();
    }

    pub fn remove_transaction(&mut self, transaction_id: Id) -> Result<(), TreasuryServiceError> {
        todo!();
    }
}
