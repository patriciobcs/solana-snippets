use solana_program::entrypoint::ProgramResult;

//* title: Get Rent Sysvar
//* description: Gets the Rent Sysvar
//* platform: native, anchor
//* category: system
//* prefix: grent
//* requires
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn processor() -> ProgramResult {
    /** content **/
    let __rent__ = Rent::get()?;
    /** content **/
    Ok(())
}
