/// title: Get Accounts Iterator
/// description: Start iterator to get accounts of instruction
/// platform: native, anchor
/// prefix: gaccs

/// processor requires
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::account_info::next_account_info;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    /* snippet */
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    /* snippet */
    Ok(())
}
