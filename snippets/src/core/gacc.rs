/**
 * title: Get Account
 * description: Get next account in the accounts iterator 
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult
};
// snippet-suggestion-end

fn gacc(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    // snippet-start
    let __account___info = next_account_info(account_info_iter)?;
    // snippet-end
    Ok(())
}
