/**
 * title: Burn Tokens
 * description: Burn tokens using the Token Program
 * platform: native, anchor
 */
// snippet-requires-start
use solana_program::program::invoke;
use solana_program::entrypoint::ProgramResult;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use spl_token::instruction::burn;
// snippet-requires-end

fn tkb(accounts: &[AccountInfo], __amount__: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let __token___info = next_account_info(account_info_iter)?;
    let __mint___info = next_account_info(account_info_iter)?;
    let __authority___info = next_account_info(account_info_iter)?;

    // snippet-body-start
    invoke(
        &burn(
            &spl_token::ID,
            __token___info.key,
            __mint___info.key,
            __authority___info.key,
            &[],
            __amount__,
        )?,
        &[
            __token___info.clone(),
            __mint___info.clone(),
            __authority___info.clone(),
        ],
    )?;
    // snippet-body-end

    Ok(())
}
