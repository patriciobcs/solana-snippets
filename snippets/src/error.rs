//! Error types

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the Custom program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum CustomError {
    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    /// Insufficient funds for the operation requested.
    #[error("Insufficient funds")]
    InsufficientFunds,
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
}
impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for CustomError {
    fn type_of() -> &'static str {
        "CustomError"
    }
}

impl PrintProgramError for CustomError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + num_traits::FromPrimitive,
    {
        match self {
            CustomError::NotRentExempt => {
                msg!("Error: Lamport balance below rent-exempt threshold")
            }
            CustomError::InsufficientFunds => msg!("Error: insufficient funds"),
            CustomError::InvalidInstruction => msg!("Error: Invalid instruction"),
        }
    }
}
