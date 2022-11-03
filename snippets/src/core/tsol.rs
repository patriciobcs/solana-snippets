/**
 * title: Transfer Native SOL
 * description: Transfer native SOL using System Program
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    system_instruction::transfer,
};
// snippet-suggestion-end

fn tsol(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __sender___info = next_account_info(account_info_iter)?;
    let __receiver___info = next_account_info(account_info_iter)?;
    let __system_program___info = next_account_info(account_info_iter)?;

    // snippet-start
    invoke(
        &transfer(__sender___info.key, __receiver___info.key, __amount__),
        &[
            __sender___info.clone(),
            __receiver___info.clone(),
            __system_program___info.clone(),
        ],
    )?;
    // snippet-end

    Ok(())
}
