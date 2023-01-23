//* title: Instruction
//* description: Template for an instruction context and its processor
//* platform: anchor
//* category: system
//* display: vscode

/*/* content */*/
use anchor_lang::prelude::*

#[derive(Accounts)]
pub struct __Instruction__ {}

pub fn processor(_ctx: Context<__Instruction__>) -> Result<()> {
    Ok(())
}
/*/* content */*/