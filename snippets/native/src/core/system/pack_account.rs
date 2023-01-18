use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Unpack Account
//* description: Unpacks an Account Info
//* platform: native, anchor
//* category: system
//* prefix: apack
//* requires
use crate::state::__Account__;
use solana_program::account_info::next_account_info;
use solana_program::program_pack::Pack;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;
    let __account__ = __Account__ {};

    /** content **/
    __Account__::pack(__account__, &mut __account___info.data.borrow_mut())?;
    /** content **/
    Ok(())
}
