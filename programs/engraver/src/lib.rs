#![deny(missing_docs)]
//! A program for engraving Solana accounts. This program writes through account data
//! and takes ownership of the account.

/// The entrypoint for the program.
pub mod entrypoint;

/// Error types returned by the program.
pub mod error;

/// Definitions for all instructions on the Engraver program.
pub mod instruction;

/// Actual instruction logic.
pub mod processor;

/// State structs for the program.
pub mod state;

pub use solana_program;

solana_program::declare_id!("ENGRVY4DL6uKDnNS91hCkJMwzTfcofYpkZH8zsgJfzA3");
