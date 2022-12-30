#![deny(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_doc_tests)]
//! Rust-based financial management service
//! 
//! Stores a database of transactions
//! 
//! ```
//! use treasury_service::TreasuryService;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let mut service = TreasuryService::new(":memory:")
//!         .await
//!         .unwrap();
//! }
//! ```

/// treasury-service data structures
mod data_structures;
mod treasury_service;
mod database_driver;

pub use crate::treasury_service::{TreasuryService, TreasuryServiceError};