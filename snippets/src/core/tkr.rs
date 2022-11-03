/**
 * title: Revoke Tokens Delegation
 * description: Revoke the delegation of any tokens using the Token Program
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
};
pub use spl_token::{instruction::revoke, ID};
// snippet-suggestion-end

fn tkr(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __token___info = next_account_info(account_info_iter)?;
    let __delegate___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    // snippet-start
    invoke(
        &revoke(
            &spl_token::ID,
            __token___info.key,
            __authority___info.key,
            &[],
        )?,
        &[
            __token___info.clone(),
            __delegate___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    // snippet-end

    Ok(())
}
