use std::collections::HashMap;

use crate::{
    account::{Account, Error},
    transaction::Transaction,
};

/// A toy payments engine
pub struct Engine {
    /// The accounts available for processing
    pub accounts: HashMap<u16, Account>,
}

impl Engine {
    /// Applies a group of transactions to their associated accounts.
    /// Returns the errors that occurred and the entry in which they
    /// occurred
    ///
    /// # Arguments
    ///
    /// `transactions`: Some container of transactions
    pub fn apply_transactions<T>(&mut self, transactions: &T) -> Vec<(usize, Error)>
    where
        for<'a> &'a T: IntoIterator<Item = &'a Transaction>,
    {
        // apply transactions to all accounts and filters out successful
        // ones, because here we are interested in the errors. this is a
        // bit fancy and could also be done more simply, but this is how
        // I like to make use of functional programming
        transactions
            .into_iter()
            .enumerate()
            .map(|(entry, t)| {
                (
                    // add 1 because it references readable entries
                    entry + 1,
                    {
                        // create the account if it does not exist
                        self.accounts
                            .entry(t.client)
                            .or_insert_with(|| Account::new(t.client))
                    }
                    .apply_transaction(t),
                )
            })
            .filter_map(|(entry, res)| res.err().map(|e| (entry, e)))
            .collect()
    }

    /// Creates a new accounts engine
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
}
