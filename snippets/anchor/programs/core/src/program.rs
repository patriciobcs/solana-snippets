//* title: Program
//* description: Creates a basic program entrypoint
//* platform: anchor
//* category: system
//* prefix: program
//* display: vscode

/*/* content */*/
use anchor_lang::prelude::*;

#[program]
pub mod anchor {
    pub use super::*;

    pub fn __instruction__(ctx: Context<__Instruction__>) -> Result<()> {
        instructions::__instruction__::processor(ctx)
    }
}
/*/* content */*/