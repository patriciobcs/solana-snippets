/// title: Get Rent Sysvar
/// description: Gets the Rent Sysvar
/// platform: native, anchor
/// prefix: grent

/// processor requires
use solana_program::entrypoint::ProgramResult;

/// snippet requires
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn processor() -> ProgramResult {
    /* snippet */
    let __rent__ = Rent::get()?;
    /* snippet */
    Ok(())
}
