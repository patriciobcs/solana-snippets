/// title: Delegate Tokens
/// description: Delegate tokens using the Token Program
/// platform: native, anchor
/// prefix: tka

/// processor requires
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::program::invoke;
use spl_token::instruction::approve;

pub fn processor(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __token___info = next_account_info(account_info_iter)?;
    let __delegate___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    /* snippet */
    invoke(
        &approve(
            &spl_token::ID,
            __token___info.key,
            __delegate___info.key,
            __authority___info.key,
            &[],
            __amount__,
        )?,
        &[
            __token___info.clone(),
            __delegate___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    /* snippet */
    Ok(())
}
