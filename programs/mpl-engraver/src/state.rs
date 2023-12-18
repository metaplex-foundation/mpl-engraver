use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::MplEngraverError;

use mpl_token_metadata::accounts::{MasterEdition, Metadata};
use spl_token::state::Account as TokenAccount;

/// A helper struct for asserting common conditions on an account.
pub struct Asserter {}

impl Asserter {
    /// Asserts that the account is a signer.
    pub fn is_signer(account_info: &AccountInfo) -> ProgramResult {
        if !account_info.is_signer {
            return Err(MplEngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }

    /// Asserts that the account is owned by the Engraver program.
    pub fn owned_by(account_info: &AccountInfo) -> ProgramResult {
        if account_info.owner != &crate::ID {
            return Err(MplEngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }

    /// Asserts that the mint matches the metadata account.
    pub fn mint_matches_metadata(mint: &Pubkey, metadata: &Metadata) -> ProgramResult {
        if mint != &metadata.mint {
            return Err(MplEngraverError::MintMetadataMismatch.into());
        }

        Ok(())
    }

    /// Asserts that the mint matches the token account.
    pub fn mint_matches_token(mint: &Pubkey, token: &TokenAccount) -> ProgramResult {
        if &token.mint != mint {
            return Err(MplEngraverError::MintTokenMismatch.into());
        }

        Ok(())
    }

    /// Asserts that the edition supply is zero so that printable editions cannot be engraved.
    pub fn edition_supply_is_zero(edition: &MasterEdition) -> ProgramResult {
        if edition.supply != 0 {
            return Err(MplEngraverError::EditionSupplyMismatch.into());
        }

        Ok(())
    }

    /// Asserts that the account is a valid metadata account.
    pub fn is_metadata_account(account_info: &AccountInfo, mint: &Pubkey) -> ProgramResult {
        let pda = Metadata::find_pda(mint).0;
        if account_info.key != &pda {
            return Err(MplEngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }

    /// Asserts that the account is a valid edition account.
    pub fn is_edition_account(account_info: &AccountInfo, mint: &Pubkey) -> ProgramResult {
        let pda = MasterEdition::find_pda(mint).0;
        if account_info.key != &pda {
            return Err(MplEngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }
}
