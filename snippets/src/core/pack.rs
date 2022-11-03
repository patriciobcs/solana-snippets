/**
 * title: Unpack Account
 * description: Unpacks an Account Info
 * platform: native, anchor
 */
// snippet-suggestion-start
use crate::state::__Account__;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::Pack,
};
// snippet-suggestion-end

fn pack(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    let __account__ = __Account__ {};

    // snippet-start
    __Account__::pack(__account__, &mut __account___info.data.borrow_mut())?;
    // snippet-end

    Ok(())
}
