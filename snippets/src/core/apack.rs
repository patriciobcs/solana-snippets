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

fn apack(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    let __account__ = __Account__ {};

    // snippet-body-start
    __Account__::pack(__account__, &mut __account___info.data.borrow_mut())?;
    // snippet-body-end



    Ok(())
}
