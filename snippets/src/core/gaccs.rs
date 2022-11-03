/**
 * title: Get Accounts Iterator
 * description: Start iterator to get accounts of instruction
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult
};
// snippet-suggestion-end

fn gaccs(accounts: &[AccountInfo]) -> ProgramResult {
    // snippet-start
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    // snippet-end
    Ok(())
}
