/**
 * title: Get Clock Sysvar
 * description: Gets the Clock Sysvar
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    entrypoint::ProgramResult,
    sysvar::{clock::Clock, Sysvar},
};
// snippet-suggestion-end

fn gclock() -> ProgramResult {
    // snippet-start
    let __clock__ = Clock::get()?;
    // snippet-end

    Ok(())
}
