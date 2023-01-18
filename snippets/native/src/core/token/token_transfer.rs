use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Transfer Tokens
//* description: Transfer tokens using the Token Program
//* platform: native, anchor
//* category: token
//* prefix: tkt
//* requires
use solana_program::program::invoke;
use spl_token::instruction::transfer;

pub fn processor(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __sender___info = next_account_info(account_info_iter)?;
    let __receiver___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    /** content **/
    invoke(
        &transfer(
            &spl_token::ID,
            __sender___info.key,
            __receiver___info.key,
            __authority___info.key,
            &[],
            __amount__,
        )?,
        &[
            __sender___info.clone(),
            __receiver___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    /** content **/
    Ok(())
}
