/// title: Get Clock Sysvar
/// description: Gets the Clock Sysvar
/// platform: native, anchor
/// prefix: gclock

/// processor requires
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::sysvar::clock::Clock;
use solana_program::sysvar::Sysvar;

pub fn processor() -> ProgramResult {
    /* snippet */
    let __clock__ = Clock::get()?;
    /* snippet */
    Ok(())
}
