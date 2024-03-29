#![deny(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_doc_tests)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
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
pub mod data_structures;
mod database_driver;
mod treasury_service;

pub use crate::treasury_service::{TreasuryService, TreasuryServiceError};
