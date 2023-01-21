//* title: Instruction
//* description: Template for an instruction context and its processor
//* platform: anchor
//* category: system
//* prefix: instruction
//* requires
use anchor_lang::prelude::Context;
use anchor_lang::prelude::Result;
use anchor_lang::Accounts;

/*/* content */*/
#[derive(Accounts)]
pub struct __Instruction__ {}

pub fn processor(_ctx: Context<__Instruction__>) -> Result<()> {
    Ok(())
}
/*/* content */*/