/**
 * title: Get Accounts Iterator
 * description: Start iterator to get accounts of instruction
 * platform: native, anchor
 */
// snippet-requires-start
use solana_program::entrypoint::ProgramResult;
use solana_program::account_info::AccountInfo;
use solana_program::account_info::next_account_info;
// snippet-requires-end

fn gaccs(accounts: &[AccountInfo]) -> ProgramResult {
    // snippet-body-start
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    // snippet-body-end
    Ok(())
}
