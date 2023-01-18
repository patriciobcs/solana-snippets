use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

//* title: Revoke Tokens Delegation
//* description: Revoke the delegation of any tokens using the Token Program
//* platform: native, anchor
//* category: token
//* prefix: tkr
//* requires
use solana_program::program::invoke;
use spl_token::instruction::revoke;

pub fn processor(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __token___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    /** content **/
    invoke(
        &revoke(
            &spl_token::ID,
            __token___info.key,
            __authority___info.key,
            &[],
        )?,
        &[
            __token___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    /** content **/
    Ok(())
}
