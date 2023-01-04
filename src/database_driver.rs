use sqlx::{Connection, Error, SqliteConnection};
use thiserror::Error;

use crate::data_structures::{DateError, Id, Transaction};

#[derive(Debug, Error)]
pub enum DatabaseDriverError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] Error),
    #[error("Date error: {0}")]
    DateError(#[from] DateError),
}

#[derive(Debug)]
pub struct DatabaseDriver {
    conn: SqliteConnection,
}

impl DatabaseDriver {
    pub async fn new(path: &str) -> Result<Self, DatabaseDriverError> {
        let mut conn = SqliteConnection::connect(path).await?;

        sqlx::query_file!("queries/database_setup.sql")
            .execute(&mut conn)
            .await?;

        Ok(Self { conn })
    }

    pub async fn get_transactions(&mut self) -> Result<Vec<Transaction>, DatabaseDriverError> {
        let query: Vec<Transaction> = sqlx::query!("SELECT * FROM transactions")
            .fetch_all(&mut self.conn)
            .await?
            .into_iter()
            .map(|r| Transaction::new(r.id as Id, r.amount as u64, r.date.into()))
            .collect();

        Ok(query)
    }

    pub async fn add_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), DatabaseDriverError> {
        let id = transaction.get_id() as i64;
        let amount = transaction.get_amount() as i64;
        let date: chrono::NaiveDateTime = transaction.get_date().try_into()?;

        sqlx::query!(
            "INSERT INTO transactions VALUES (?, ?, ?)",
            id,
            amount,
            date
        )
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    pub async fn get_next_id(&mut self) -> Result<Id, DatabaseDriverError> {
        Ok(sqlx::query!("SELECT MAX(id) as maxid FROM transactions")
            .fetch_one(&mut self.conn)
            .await?
            .maxid
            .map(|x| x + 1)
            .unwrap_or(0) as u32)
    }

    pub async fn set_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), DatabaseDriverError> {
        let id = transaction.get_id() as i64;
        let amount = transaction.get_amount() as i64;
        let date: chrono::NaiveDateTime = transaction.get_date().try_into()?;

        sqlx::query!(
            "UPDATE transactions SET amount=?, date=? WHERE id=?",
            amount,
            date,
            id
        )
        .execute(&mut self.conn)
        .await?;

        Ok(())
    }

    pub async fn remove_transaction(&mut self, id: Id) -> Result<(), DatabaseDriverError> {
        sqlx::query!("DELETE FROM transactions WHERE id=?", id)
            .execute(&mut self.conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use crate::data_structures::Date;

    use super::*;

    async fn create_in_memory_database() -> DatabaseDriver {
        DatabaseDriver::new(":memory:").await.unwrap()
    }

    fn create_dummy_transaction(id: u32) -> Transaction {
        Transaction::new(id, 105, Date::new(1, 1, 2023))
    }

    #[tokio::test]
    async fn new() {
        assert!(DatabaseDriver::new(":memory:").await.is_ok());
        assert!(DatabaseDriver::new("test.db").await.is_err());
    }

    #[tokio::test]
    async fn add_transaction() {
        let mut database = create_in_memory_database().await;

        let transaction = create_dummy_transaction(0);
        database.add_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn get_transactions() {
        let mut database = create_in_memory_database().await;

        let transactions_1 = database.get_transactions().await.unwrap();
        assert_eq!(transactions_1, vec![]);
    }

    #[tokio::test]
    async fn set_transaction() {
        let mut database = create_in_memory_database().await;

        let transaction = create_dummy_transaction(1);
        database.add_transaction(transaction.clone()).await.unwrap();
        database.set_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn remove_transaction() {
        let mut database = create_in_memory_database().await;

        let transaction = create_dummy_transaction(0);
        database.add_transaction(transaction).await.unwrap();
        database.remove_transaction(0).await.unwrap();
    }
}
