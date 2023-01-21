use solana_program::entrypoint::ProgramResult;

//* title: Get Clock Sysvar
//* description: Gets the Clock Sysvar
//* platform: native, anchor
//* category: system
//* prefix: gclock
//* requires
use solana_program::sysvar::clock::Clock;
use solana_program::sysvar::Sysvar;

pub fn processor() -> ProgramResult {
    /*/* content */*/
    let __clock__ = Clock::get()?;
    /*/* content */*/
    Ok(())
}
