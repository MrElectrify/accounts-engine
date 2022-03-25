use std::{env, io};

use csv::{ReaderBuilder, Trim, Writer};
use fallible_iterator::FallibleIterator;
use transaction::Transaction;

use crate::engine::Engine;

mod account;
mod engine;
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
    // create the engine
    let mut engine = Engine::new();
    // apply the transactions
    let errs = engine.apply_transactions(&transactions);
    if !errs.is_empty() {
        eprintln!(
            "The following {} errors occurred while applying transactions:",
            errs.len()
        );
        for (entry, err) in errs {
            eprintln!("Entry {}: {}", entry, err);
        }
    }
    // generate the output CSV
    let mut writer = Writer::from_writer(io::stdout());
    // output all accounts
    for (id, account) in &engine.accounts {
        // print errors that may happen in serialization
        if let Err(e) = writer.serialize(account) {
            eprintln!("Failed to serialize account {} to CSV: {}", id, e);
        }
    }
    // make sure all CSV is output to stdout
    if let Err(e) = writer.flush() {
        eprint!("Failed to write accounts to stdout: {}", e);
    }
}
