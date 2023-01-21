use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Unpack Account
//* description: Unpacks an Account Info
//* platform: native, anchor
//* category: system
//* prefix: aunpack
//* requires
use crate::state::__Account__;
use solana_program::account_info::next_account_info;
use solana_program::program_pack::Pack;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;

    /*/* content */*/
    let mut __account__ = __Account__::unpack(&__account___info.data.borrow())?;
    /*/* content */*/
    Ok(())
}