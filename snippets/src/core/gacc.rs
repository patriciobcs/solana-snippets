/// title: Get Account
/// description: Get next account in the accounts iterator
/// platform: native, anchor
/// prefix: gacc

/// processor requires
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::account_info::next_account_info;

fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    /* snippet */
    let __account___info = next_account_info(account_info_iter)?;
    /* snippet */
    Ok(())
}
