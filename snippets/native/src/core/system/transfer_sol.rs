use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Transfer Native SOL
//* description: Transfer native SOL using System Program
//* platform: native, anchor
//* category: system
//* prefix: tsol
//* requires
use solana_program::program::invoke;
use solana_program::system_instruction::transfer;

pub fn processor(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __sender___info = next_account_info(account_info_iter)?;
    let __receiver___info = next_account_info(account_info_iter)?;
    let __system_program___info = next_account_info(account_info_iter)?;

    /** content **/
    invoke(
        &transfer(__sender___info.key, __receiver___info.key, __amount__),
        &[
            __sender___info.clone(),
            __receiver___info.clone(),
            __system_program___info.clone(),
        ],
    )?;
    /** content **/
    Ok(())
}
