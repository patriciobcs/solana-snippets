/// title: Init Associated Token Account
/// description: Initialize an associated token account using the Token Program
/// platform: native, anchor
/// prefix: tki

/// processor requires
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::program::invoke;
use spl_token::instruction::initialize_account;

pub fn processor(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __token___info = next_account_info(account_info_iter)?;
    let __mint___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    /* snippet */
    invoke(
        &initialize_account(
            &spl_token::ID,
            __token___info.key,
            __mint___info.key,
            __authority___info.key,
        )?,
        &[
            __token___info.clone(),
            __mint___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    /* snippet */
    Ok(())
}
