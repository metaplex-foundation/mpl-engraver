#![allow(missing_docs)]
use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

/// Options for which account to engrave.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum EngraveTarget {
    /// Engrave the metadata account.
    Metadata,
    /// Engrave the edition account.
    Edition,
}

/// Engrave IX arguments
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EngraveArgs {
    /// Which account is being engraved.
    pub target: EngraveTarget,
    /// The data to write to the account.
    pub data: Vec<u8>,
}

/// Instructions supported by the Engraver program. Currently the only necessary one is Engrave.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum EngraverInstruction {
    /// Write through account data and take ownership.
    #[account(0, writable, signer, name="authority", desc = "NFT update authority")]
    #[account(1, name="mint", desc = "NFT mint account")]
    #[account(2, writable, name="metadata", desc = "NFT metadata account")]
    #[account(3, writable, name="edition", desc = "NFT edition account")]
    #[account(4, name="system_program", desc = "System program")]
    Engrave(EngraveArgs),
}
