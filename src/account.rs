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
    /// The transactions that have been applied to the account and
    /// can be disputed
    #[serde(skip)]
    transactions: HashMap<u32, Transaction>,
}

/// Any error that arises during transaction processing.
/// No comments because errors are descriptive
#[derive(Debug, Error, PartialEq)]
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
            transactions: HashMap::new(),
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
            Type::Deposit => self.deposit(transaction)?,
            Type::Withdrawal => self.withdrawal(transaction)?,
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
        if let Some(transaction) = self.transactions.remove(&tx) {
            // if a withdrawal is charged back, ignore it. this is an
            // assumption that it is not possible to chargeback a withdrawal,
            // that would make no sense
            if transaction.r#type == Type::Withdrawal {
                return;
            }
            // as stated above, remove the funds from total and held
            self.held -= transaction.amount.unwrap();
            self.total -= transaction.amount.unwrap();
            self.locked = true;
        }
    }

    /// Deposit funds into an account. Takes the transaction
    /// directly because it must be tracked in case of dispute
    ///
    /// # Arguments
    ///
    /// `transaction`: The transaction associated with the deposit
    fn deposit(&mut self, transaction: &Transaction) -> Result<(), Error> {
        // make sure there is an associated amount
        let amount = transaction.amount.ok_or(Error::MissingAmount)?;
        self.available += amount;
        self.total += amount;
        // track the transaction in case of dispute
        self.transactions
            .insert(transaction.tx, transaction.clone());
        Ok(())
    }

    /// Disputes a transaction
    ///
    /// # Arguments
    ///
    /// `tx`: The referenced transaction identifier
    fn dispute(&mut self, tx: u32) {
        // per instructions, ignore transactions that cannot be found
        if let Some(transaction) = self.transactions.get(&tx) {
            // hold the amount disputed
            self.held += transaction.amount.unwrap();
            match transaction.r#type {
                Type::Deposit => {
                    // we must reclaim the funds they have deposited
                    self.available -= transaction.amount.unwrap();
                }
                Type::Withdrawal => {
                    // they withdrew funds, but held has increased
                    self.total += transaction.amount.unwrap();
                }
                _ => {}
            }
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
        if let Some(transaction) = self.transactions.remove(&tx) {
            // move the funds from held to available
            self.held -= transaction.amount.unwrap();
            self.available += transaction.amount.unwrap();
        }
    }

    /// Withdrawal funds from an account. Takes the transaction
    /// directly because it must be tracked in case of dispute
    ///
    /// # Arguments
    ///
    /// `transaction`: The transaction associated with the withdrawal
    fn withdrawal(&mut self, transaction: &Transaction) -> Result<(), Error> {
        // make sure they have an amount associated
        let requested = transaction.amount.ok_or(Error::MissingAmount)?;
        if requested > self.available {
            Err(Error::InsufficientFunds(requested, self.available))
        } else {
            self.available -= requested;
            self.total -= requested;
            // track the transaction in case of dispute
            self.transactions
                .insert(transaction.tx, transaction.clone());
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        account::Error,
        transaction::{Transaction, Type},
    };

    use super::Account;

    /// Test regular deposits
    #[test]
    fn deposit() {
        let mut acc = Account::new(1);
        acc.deposit(&Transaction {
            r#type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Some(12.2233),
        })
        .unwrap();
        assert_eq!(acc.available, 12.2233);
        assert_eq!(acc.total, 12.2233);
        assert_eq!(acc.transactions.len(), 1);
    }

    /// Test insufficient funds while withdrawaling funds
    #[test]
    fn withdrawal_insufficient_funds() {
        let mut acc = Account::new(1);
        assert_eq!(
            acc.withdrawal(&Transaction {
                r#type: Type::Withdrawal,
                client: 1,
                tx: 1,
                amount: Some(123.45),
            })
            .unwrap_err(),
            Error::InsufficientFunds(123.45, 0.0)
        );
        // make sure we didn't track this transaction
        // that did not do anything
        assert!(acc.transactions.is_empty());
    }

    /// Test withdrawal sanity after an equal deposit
    #[test]
    fn withdrawal_sanity() {
        let amount = 20.924;
        let mut acc = Account::new(1);
        acc.deposit(&Transaction {
            r#type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Some(amount),
        })
        .unwrap();
        acc.withdrawal(&Transaction {
            r#type: Type::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(amount),
        })
        .unwrap();
        assert_eq!(acc.available, 0.0);
        assert_eq!(acc.total, 0.0);
        assert_eq!(acc.transactions.len(), 2);
    }

    /// Test dispute chargeback and account freeze
    #[test]
    fn dispute_chargeback() {
        let amount = 20.924;
        let mut acc = Account::new(1);
        acc.deposit(&Transaction {
            r#type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Some(amount),
        })
        .unwrap();
        acc.dispute(1);
        assert_eq!(acc.held, amount);
        assert_eq!(acc.total, amount);
        acc.chargeback(1);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.total, 0.0);
        assert!(acc.locked);
        assert!(acc.transactions.is_empty());
    }

    /// Test dispute resolution
    #[test]
    fn dispute_resolve() {
        let amount = 20.924;
        let mut acc = Account::new(1);
        acc.deposit(&Transaction {
            r#type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Some(amount),
        })
        .unwrap();
        acc.withdrawal(&Transaction {
            r#type: Type::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(amount),
        })
        .unwrap();
        acc.dispute(2);
        assert_eq!(acc.held, amount);
        assert_eq!(acc.total, amount);
        acc.resolve(2);
        assert_eq!(acc.held, 0.0);
        assert_eq!(acc.total, amount);
        assert_eq!(acc.transactions.len(), 1);
    }

    /// Ensure account locking works
    #[test]
    fn locked_account() {
        let mut acc = Account::new(1);
        acc.locked = true;
        assert_eq!(
            acc.apply_transaction(&Transaction {
                r#type: Type::Deposit,
                client: 1,
                tx: 1,
                amount: Some(1.0),
            })
            .unwrap_err(),
            Error::AccountLocked
        );
        assert_eq!(
            acc.apply_transaction(&Transaction {
                r#type: Type::Deposit,
                client: 1,
                tx: 2,
                amount: Some(1.0),
            })
            .unwrap_err(),
            Error::AccountLocked
        );
    }
}
