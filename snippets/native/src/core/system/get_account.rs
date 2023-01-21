use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Get Account
//* description: Get next account in the accounts iterator
//* platform: native
//* category: system
//* prefix: gacc
//* requires
use solana_program::account_info::next_account_info;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    /*/* content */*/
    let __account___info = next_account_info(account_info_iter)?;
    /*/* content */*/
    Ok(())
}
