use serde_derive::Serialize;
use thiserror::Error;

use crate::transaction::Transaction;

/// A client's account
#[derive(Debug, Serialize)]
pub struct Account {
    /// The owning client's identifier
    client: u16,
    /// The amount of available funds
    available: f64,
    /// The amount of held funds
    held: f64,
    /// The total amount of funds
    total: f64,
    /// True if the account is locked
    locked: bool,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("The transaction could not be applied because the account was locked")]
    AccountLocked,
}

impl Account {
    /// Applies the transaction to the account.
    ///
    /// # Arguments
    ///
    /// `transaction`: The transaction to apply
    pub fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), Error> {
        Ok(())
    }
    /// Creates a new account for a client
    ///
    /// # Arguments
    ///
    /// `client`: The owning client's identifier
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
