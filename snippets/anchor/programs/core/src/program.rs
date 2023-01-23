//* title: Program
//* description: Creates a basic program entrypoint
//* platform: anchor
//* category: system
//* prefix: program
//* display: vscode

/*/* content */*/
mod instructions;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor {
    pub use super::*;

    pub fn __instruction__(ctx: Context<__Instruction__>) -> Result<()> {
        instructions::__instruction__::processor(ctx)
    }
}
/*/* content */*/