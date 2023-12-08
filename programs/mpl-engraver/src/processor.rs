use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::accounts::{MasterEdition, Metadata};
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use spl_token::state::Account as TokenAccount;

use crate::instruction::accounts::EngraveAccounts;
use crate::instruction::EngraverInstruction;
use crate::state::Asserter;

const MAX_METADATA_LEN: usize = 679;

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

fn process_engrave<'a>(accounts: &'a [AccountInfo<'a>], mut data: Vec<u8>) -> ProgramResult {
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
    Asserter::is_signer(&edition_info)?;
    // Authority must be a pass-through signer to prove the update authority is making the request.
    msg!("Checking authority is signer");
    Asserter::is_signer(&authority_info)?;

    // Metadata and Edition must be the PDAs on Token Metadata derived from the mint, but must be
    // owned by the Engraver program.
    // These checks can be replaced by from_account_info which should validate both.
    msg!("Checking metadata and edition are owned by engraver");
    Asserter::owned_by(&metadata_info)?;
    // Asserter::owned_by(edition_info)?;
    Asserter::is_metadata_account(&metadata_info, mint_info.key)?;
    Asserter::is_edition_account(&edition_info, mint_info.key)?;

    msg!("Finished checks.");

    // Reallocate the metadata account to the correct length.
    metadata_info.realloc(679, false)?;

    let nft_metadata = Metadata::safe_deserialize(&data)?;
    msg!("Deserialized metadata.");
    msg!("metadata: {:?}", nft_metadata);

    let edition = MasterEdition::try_from_slice(&edition_info.data.borrow())?;
    msg!("Deserialized edition.");

    let token = TokenAccount::unpack(&token_info.data.borrow())?;
    msg!("Deserialized token.");

    Asserter::mint_matches_metadata(mint_info.key, &nft_metadata)?;
    Asserter::mint_matches_token(mint_info.key, &token)?;
    Asserter::edition_supply_is_zero(&edition)?;
    msg!("Finished relationship checks.");

    // TODO: Checks: The rest of the owl

    msg!("Writing metadata under the new owning program...");
    // Set the metadata under the new owner, now it cannot be modifed by Token Metadata any more.
    let mut bytes = Vec::with_capacity(MAX_METADATA_LEN);
    BorshSerialize::serialize(&nft_metadata, &mut bytes)?;
    data[..bytes.len()].copy_from_slice(&bytes);

    Ok(())
}
