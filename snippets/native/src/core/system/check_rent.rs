use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::sysvar::rent::Rent;

//* title: Check Rent Exempt
//* description: Check if an account is rent exempt
//* platform: native, anchor
//* category: system
//* prefix: chrent
//* requires
use crate::state::__Account__;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __account___info = next_account_info(account_info_iter)?;

    let rent = Rent::get()?;

    /** content **/
    if !rent.is_exempt(__account___info.lamports(), __Account__::LEN) {
        return Err(ProgramError::InvalidAccountData.into());
    }
    /** content **/
    Ok(())
}
