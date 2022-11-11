/**
 * title: Get Rent Sysvar
 * description: Gets the Rent Sysvar
 * platform: native, anchor
 */
// snippet-requires-start
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::entrypoint::ProgramResult;
// snippet-requires-end

pub fn grent() -> ProgramResult {
    // snippet-body-start
    let __rent__ = Rent::get()?;
    // snippet-body-end

    Ok(())
}
