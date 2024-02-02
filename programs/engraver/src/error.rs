use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Engraver program.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum MplEngraverError {
    /// The instruction passed in was invalid.
    #[error("Invalid Instruction")]
    InvalidInstruction = 0,
    /// The account was not owned by the expected program.
    #[error("Invalid Account owner")]
    InvalidAccountOwner = 1,
    /// The mint does not match the metadata account.
    #[error("The mint does not match the metadata account")]
    MintMetadataMismatch = 2,
    /// The mint does not match the token account.
    #[error("The mint does not match the token account")]
    MintTokenMismatch = 3,
    /// The edition supply is not zero.
    #[error("The edition supply is not zero")]
    EditionSupplyMismatch = 4,
    /// The derived PDA is not valid.
    #[error("The derived PDA is not valid")]
    DerivedKeyInvalid = 5,
}

impl PrintProgramError for MplEngraverError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MplEngraverError> for ProgramError {
    fn from(e: MplEngraverError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MplEngraverError {
    fn type_of() -> &'static str {
        "Mpl Engraver Error"
    }
}
