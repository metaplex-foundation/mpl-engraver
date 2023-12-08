use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum EngraverError {
    #[error("Invalid Instruction")]
    InvalidInstruction = 0,
    #[error("Invalid Account owner")]
    InvalidAccountOwner = 1,
    #[error("The mint does not match the metadata account")]
    MintMetadataMismatch = 2,
    #[error("The mint does not match the token account")]
    MintTokenMismatch = 3,
    #[error("The edition supply is not zero")]
    EditionSupplyMismatch = 4,
}

impl PrintProgramError for EngraverError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<EngraverError> for ProgramError {
    fn from(e: EngraverError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for EngraverError {
    fn type_of() -> &'static str {
        "Mpl Engraver Error"
    }
}
