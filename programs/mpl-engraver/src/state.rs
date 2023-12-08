use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::EngraverError;

use mpl_token_metadata::accounts::{MasterEdition, Metadata};
use spl_token::state::Account as TokenAccount;

pub struct Asserter {}

impl Asserter {
    pub fn is_signer(account_info: &AccountInfo) -> ProgramResult {
        if !account_info.is_signer {
            return Err(EngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }
    pub fn owned_by(account_info: &AccountInfo) -> ProgramResult {
        if account_info.owner != &crate::ID {
            return Err(EngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }
    pub fn mint_matches_metadata(mint: &Pubkey, metadata: &Metadata) -> ProgramResult {
        if mint != &metadata.mint {
            return Err(EngraverError::MintMetadataMismatch.into());
        }

        Ok(())
    }
    pub fn mint_matches_token(mint: &Pubkey, token: &TokenAccount) -> ProgramResult {
        if &token.mint != mint {
            return Err(EngraverError::MintTokenMismatch.into());
        }

        Ok(())
    }
    pub fn edition_supply_is_zero(edition: &MasterEdition) -> ProgramResult {
        if edition.supply != 0 {
            return Err(EngraverError::EditionSupplyMismatch.into());
        }

        Ok(())
    }
    pub fn is_metadata_account(account_info: &AccountInfo, mint: &Pubkey) -> ProgramResult {
        let pda = Metadata::find_pda(mint).0;
        if account_info.key != &pda {
            return Err(EngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }

    pub fn is_edition_account(account_info: &AccountInfo, mint: &Pubkey) -> ProgramResult {
        let pda = MasterEdition::find_pda(mint).0;
        if account_info.key != &pda {
            return Err(EngraverError::InvalidAccountOwner.into());
        }

        Ok(())
    }
}
