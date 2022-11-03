use crate::state::__Account__;
/**
 * title: Check Rent Exempt
 * description: Check if an Account is Rent Exempt
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    sysvar::{rent::Rent, Sysvar},
};
// snippet-suggestion-end

fn chrent(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;

    let rent = Rent::get()?;

    // snippet-start
    if !rent.is_exempt(__account___info.lamports(), __Account__::LEN) {
        return Err(ProgramError::InvalidAccountData.into());
    }
    // snippet-end

    Ok(())
}
