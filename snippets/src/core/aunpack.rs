/**
 * title: Unpack Account
 * description: Unpacks an Account Info
 * platform: native, anchor
 */
// snippet-requires-start
use crate::state::__Account__;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_pack::Pack;
// snippet-requires-end

fn aunpack(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    // snippet-body-start
    let mut __account__ = __Account__::unpack(&__account___info.data.borrow())?;
    // snippet-body-end

    Ok(())
}
