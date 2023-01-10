pub mod core;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

solana_program::declare_id!("CzwQqKgUaDADfG2oHCm3UNuyqMCE871yqrz7WiMyGhS8");
