/// title: Unpack Account
/// description: Unpacks an Account Info
/// platform: native, anchor
/// prefix: apack

/// processor requires
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use crate::state::__Account__;
use solana_program::account_info::next_account_info;
use solana_program::program_pack::Pack;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    let __account__ = __Account__ {};

    /* snippet */
    __Account__::pack(__account__, &mut __account___info.data.borrow_mut())?;
    /* snippet */
    Ok(())
}
