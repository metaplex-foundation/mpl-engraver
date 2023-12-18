use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::accounts::{MasterEdition, Metadata};
use mpl_utils::{assert_derivation, assert_owned_by, assert_signer};
use solana_program::program_memory::sol_memcpy;
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use spl_token::state::Account as TokenAccount;

use crate::error::MplEngraverError;
use crate::instruction::accounts::EngraveAccounts;
use crate::instruction::{EngraveArgs, EngraveTarget, EngraverInstruction};

pub(crate) fn process_instruction<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: EngraverInstruction = EngraverInstruction::try_from_slice(instruction_data)?;

    match instruction {
        EngraverInstruction::Engrave(data) => {
            msg!("Instruction: Engrave");
            process_engrave(accounts, data)
        }
    }
}

fn process_engrave<'a>(accounts: &'a [AccountInfo<'a>], args: EngraveArgs) -> ProgramResult {
    let ctx = EngraveAccounts::context(accounts)?;
    msg!("Engraving NFT...");
    msg!("accounts: {:?}", accounts.len());
    let authority_info = ctx.accounts.authority.clone();
    let mint_info = ctx.accounts.mint.clone();
    let token_info = ctx.accounts.token.clone();
    let metadata_info = ctx.accounts.metadata.clone();
    let edition_info = ctx.accounts.edition.clone();
    let _system_program_info = ctx.accounts.system_program.clone();

    // Checks

    // Must be signed by the Edition account to prove the call came from Token Metadata.
    msg!("Checking edition is signer");
    assert_signer(&edition_info)?;
    // Authority must be a pass-through signer to prove the update authority is making the request.
    msg!("Checking authority is signer");
    assert_signer(&authority_info)?;

    assert_derivation(
        &mpl_token_metadata::ID,
        &metadata_info,
        &[
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint_info.key.as_ref(),
        ],
        MplEngraverError::InvalidAccountOwner,
    )?;
    assert_derivation(
        &mpl_token_metadata::ID,
        &metadata_info,
        &[
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint_info.key.as_ref(),
            "edition".as_bytes(),
        ],
        MplEngraverError::InvalidAccountOwner,
    )?;

    msg!("Finished common checks.");

    let nft_metadata = Metadata::safe_deserialize(&args.data)?;
    msg!("Deserialized metadata.");
    msg!("metadata: {:?}", nft_metadata);
    let serialized_data = nft_metadata.try_to_vec()?;

    // The PDAs on Token Metadata derived from the mint, but must be
    // owned by the Engraver program.
    // These checks can be replaced by from_account_info which should validate both.
    match args.target {
        EngraveTarget::Metadata => {
            msg!("Checking metadata is owned by Engraver");
            assert_owned_by(
                &metadata_info,
                &crate::ID,
                MplEngraverError::InvalidAccountOwner,
            )?;
            msg!("Engraving metadata...");
            // Reallocate the metadata account to the correct length.
            metadata_info.realloc(serialized_data.len(), false)?;
        }
        EngraveTarget::Edition => {
            msg!("Checking edition is owned by Engraver");
            assert_owned_by(
                &edition_info,
                &crate::ID,
                MplEngraverError::InvalidAccountOwner,
            )?;
            msg!("Engraving edition...");
            // Reallocate the edition account to the correct length.
            edition_info.realloc(serialized_data.len(), false)?;
        }
    }

    let edition = MasterEdition::try_from_slice(&edition_info.data.borrow())?;
    msg!("Deserialized edition.");

    let token = TokenAccount::unpack(&token_info.data.borrow())?;
    msg!("Deserialized token.");

    if nft_metadata.mint != *mint_info.key {
        return Err(MplEngraverError::MintMetadataMismatch.into());
    }
    if token.mint != *mint_info.key {
        return Err(MplEngraverError::MintTokenMismatch.into());
    }
    if edition.supply != 0 {
        return Err(MplEngraverError::EditionSupplyMismatch.into());
    }
    msg!("Finished relationship checks.");

    // TODO: Checks: The rest of the owl

    msg!("Writing metadata under the new owning program...");

    // Set the account under the new owner, now it cannot be modified by Token Metadata any more.
    match args.target {
        EngraveTarget::Metadata => {
            sol_memcpy(
                &mut metadata_info.data.borrow_mut(),
                &serialized_data,
                serialized_data.len(),
            );
        }
        EngraveTarget::Edition => {
            sol_memcpy(
                &mut edition_info.data.borrow_mut(),
                &serialized_data,
                serialized_data.len(),
            );
        }
    }

    Ok(())
}
