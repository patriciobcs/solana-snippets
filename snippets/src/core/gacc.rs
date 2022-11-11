/**
 * title: Get Account
 * description: Get next account in the accounts iterator
 * platform: native, anchor
 */
// snippet-requires-start
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
// snippet-requires-end

fn gacc(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    // snippet-body-start
    let __account___info = next_account_info(account_info_iter)?;
    // snippet-body-end
    Ok(())
}
