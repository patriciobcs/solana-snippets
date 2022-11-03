/**
 * title: Get Rent Sysvar
 * description: Gets the Rent Sysvar
 * platform: native, anchor
 */
// snippet-suggestion-start
use solana_program::{
    entrypoint::ProgramResult,
    sysvar::{rent::Rent, Sysvar},
};
// snippet-suggestion-end

pub fn grent() -> ProgramResult {
    // snippet-start
    let __rent__ = Rent::get()?;
    // snippet-end

    Ok(())
}
