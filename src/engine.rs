use std::collections::HashMap;

use crate::{
    account::{Account, Error},
    transaction::Transaction,
};

/// A toy payments engine
pub struct Engine {
    /// The accounts available for processing
    accounts: HashMap<u16, Account>,
}

impl Engine {
    /// Applies a group of transactions to their associated accounts.
    /// Returns the errors that occurred and the entry in which they
    /// occurred
    ///
    /// # Arguments
    ///
    /// `iter`: An iterator of transactions
    pub fn apply_transactions<I>(iter: I) -> Vec<(u64, Error)>
    where
        I: Iterator<Item = Transaction>,
    {
        Vec::new()
    }

    /// Creates a new accounts engine
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
}
