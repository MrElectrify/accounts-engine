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
mod test {
    use std::io::Cursor;

    use csv::{ReaderBuilder, Trim};
    use fallible_iterator::FallibleIterator;

    use super::Transaction;

    #[test]
    fn simple_deserialize() {
        let buf = include_bytes!("../test/simple.csv");
        // ignore whitespaces as specified
        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .from_reader(Cursor::new(buf));
        let transactions: Vec<Transaction> = fallible_iterator::convert(reader.records())
            .map(|record| record.deserialize(None))
            .collect()
            .unwrap();
        // make sure there are 5 transactions
        assert_eq!(transactions.len(), 5);
    }

    #[test]
    fn complex_deserialize() {
        let buf = include_bytes!("../test/complex.csv");
        // ignore whitespaces as specified
        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .from_reader(Cursor::new(buf));
        let transactions: Vec<Transaction> = fallible_iterator::convert(reader.records())
            .map(|record| record.deserialize(None))
            .collect()
            .unwrap();
        // make sure there are 5 transactions
        assert_eq!(transactions.len(), 9);
    }
}
