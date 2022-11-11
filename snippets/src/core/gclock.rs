/**
 * title: Get Clock Sysvar
 * description: Gets the Clock Sysvar
 * platform: native, anchor
 */
// snippet-requires-start
use solana_program::sysvar::Sysvar;
use solana_program::sysvar::clock::Clock;
use solana_program::entrypoint::ProgramResult;
// snippet-requires-end

fn gclock() -> ProgramResult {
    // snippet-body-start
    let __clock__ = Clock::get()?;
    // snippet-body-end

    Ok(())
}
