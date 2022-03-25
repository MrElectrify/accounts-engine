use serde_derive::Deserialize;

/// The type of a transaction. Types are aliased because
/// we assume they will be with this capitalization
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Type {
    /// A credit to the client's account
    #[serde(alias = "deposit")]
    Deposit,
    /// A debit to the client's account
    #[serde(alias = "withdrawal")]
    Withdrawal,
    /// A claim that the transaction was erroneous
    #[serde(alias = "dispute")]
    Dispute,
    /// A resolution to a dispute
    #[serde(alias = "resolve")]
    Resolve,
    /// A client's reversal to a transaction
    #[serde(alias = "chargeback")]
    Chargeback,
}

/// An actual transaction
#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    /// The type of the transaction
    pub r#type: Type,
    /// The client identifier that the transaction belongs to
    pub client: u16,
    /// The transaction identifier, likely unique
    pub tx: u32,
    /// The amount involved in the transaction
    pub amount: Option<f64>,
}

#[cfg(test)]
mod test {}
