use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum EngraverInstruction {
    /// Create My Account.
    /// A detailed description of the instruction.
    #[account(0, writable, signer, name="authority", desc = "NFT update authority")]
    #[account(1, writable, name="mint", desc = "NFT mint account")]
    #[account(2, writable, name="token", desc = "NFT token account")]
    #[account(3, writable, name="metadata", desc = "NFT metadata account")]
    #[account(4, writable, name="edition", desc = "NFT edition account")]
    #[account(5, name="system_program", desc = "System program")]
    Engrave(Vec<u8>),
}
