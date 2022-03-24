use std::env;

use csv::{ReaderBuilder, Trim};
use fallible_iterator::FallibleIterator;
use transaction::Transaction;

mod account;
mod transaction;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <transaction_csv:path>", args[0]);
        return;
    }
    // open the specified file into a reader. ignore
    // whitespaces as specified
    let mut reader = match ReaderBuilder::new().trim(Trim::All).from_path(&args[1]) {
        Ok(reader) => reader,
        Err(e) => {
            eprint!("Failed to open file {}: {}", args[1], e);
            return;
        }
    };
    // prevent unnecessary mutability with a one-liner
    let transactions: Vec<Transaction> = match fallible_iterator::convert(reader.records())
        .map(|res| res.deserialize::<Transaction>(None))
        .collect()
    {
        Ok(transactions) => transactions,
        Err(e) => {
            eprintln!("Failed to deserialize transactions: {}", e);
            return;
        }
    };
    println!("{:?}", transactions);
}
