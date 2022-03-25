use std::collections::HashMap;

use serde_derive::Serialize;
use thiserror::Error;

use crate::transaction::{Transaction, Type};

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
    /// The withdrawals that have been applied to the account and
    /// can be disputed
    #[serde(skip)]
    withdrawals: HashMap<u32, Transaction>,
}

/// Any error that arises during transaction processing.
/// No comments because errors are descriptive
#[derive(Debug, Error)]
pub enum Error {
    #[error("The transaction could not be applied because the account was locked")]
    AccountLocked,
    #[error(
        "The transaction could not be completed because the account \
        had insufficient funds. Requested: {0}, Available: {1}"
    )]
    InsufficientFunds(f64, f64),
    #[error(
        "The transaction could not be processed because it was missing an amount where expected"
    )]
    MissingAmount,
}

impl Account {
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
            withdrawals: HashMap::new(),
        }
    }

    /// Applies the transaction to the account.
    ///
    /// # Arguments
    ///
    /// `transaction`: The transaction to apply
    pub fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), Error> {
        // if the user's account is frozen, don't allow any
        // transaction to apply
        if self.locked {
            return Err(Error::AccountLocked);
        }
        // track deposits and withrawals
        match transaction.r#type {
            Type::Deposit => self.deposit(transaction.amount.ok_or(Error::MissingAmount)?),
            Type::Withdrawal => {
                self.withdrawal(transaction.amount.ok_or(Error::MissingAmount)?)?;
                // insert a successful transaction here. we assume we don't want to track
                // transactions with insufficient funds, because there was nothing applied
                // to the account
                self.withdrawals.insert(transaction.tx, transaction.clone());
            }
            Type::Dispute => self.dispute(transaction.tx),
            Type::Resolve => self.resolve(transaction.tx),
            Type::Chargeback => self.chargeback(transaction.tx),
        };
        Ok(())
    }

    /// Perform a chargeback on the account. Remove the funds associated with
    /// the transaction and freeze the account
    ///
    /// # Arguments
    ///
    /// `tx`: The referenced transaction identifier
    fn chargeback(&mut self, tx: u32) {
        // per instructions, ignore transactions that cannot be found.
        // remove it because no further action can be done, this is resolved
        if let Some(transaction) = self.withdrawals.remove(&tx) {
            // as stated above, remove the funds from total and held
            self.held -= transaction.amount.unwrap();
            self.total -= transaction.amount.unwrap();
        }
    }

    /// Deposit funds into an account
    ///
    /// # Arguments
    ///
    /// `amount`: The amount to deposit into the account
    fn deposit(&mut self, amount: f64) {
        self.available += amount;
        self.total += amount;
    }

    /// Disputes a transaction
    ///
    /// # Arguments
    ///
    /// `tx`: The referenced transaction identifier
    fn dispute(&mut self, tx: u32) {
        // per instructions, ignore transactions that cannot be found
        if let Some(transaction) = self.withdrawals.get(&tx) {
            // we already withdrew the funds, so put them back into total and held
            // we also already verified that amount is present, no need to check
            self.held += transaction.amount.unwrap();
            self.total += transaction.amount.unwrap();
        }
    }

    /// Resolves a disputed transaction, releasing the client
    /// the associated funds
    ///
    /// # Arguments
    ///
    /// `tx`: The referenced transaction identifier
    fn resolve(&mut self, tx: u32) {
        // per instructions, ignore transactions that cannot be found
        // remove it because no further action can be done, this is resolved
        if let Some(transaction) = self.withdrawals.remove(&tx) {
            // move the funds from held to available
            self.held -= transaction.amount.unwrap();
            self.available += transaction.amount.unwrap();
        }
    }

    /// Withdrawal funds from an account
    ///
    /// # Arguments
    ///
    /// `requested`: The amount of money requested to be withdrawn
    fn withdrawal(&mut self, requested: f64) -> Result<(), Error> {
        if requested > self.available {
            Err(Error::InsufficientFunds(requested, self.available))
        } else {
            self.available -= requested;
            self.total -= requested;
            Ok(())
        }
    }
}
