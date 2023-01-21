//* title: Program
//* description: Creates a basic program entrypoint
//* platform: anchor
//* category: system
//* prefix: program
//* scope: stmt
//* requires
use anchor_lang::prelude::*;

/*/* content */*/
#[program]
pub mod anchor {
    pub use super::*;

    pub fn __instruction__(ctx: Context<__Instruction__>) -> Result<()> {
        instructions::__instruction__::processor(ctx)
    }
}
/*/* content */*/