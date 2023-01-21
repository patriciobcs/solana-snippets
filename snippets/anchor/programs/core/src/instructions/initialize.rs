//* title: Check Rent Exempt
//* description: Check if an account is rent exempt
//* platform: native, anchor
//* category: system
//* prefix: chrent
//* requires
use anchor_lang::prelude::Context;
use anchor_lang::prelude::Result;
use anchor_lang::Accounts;

/*/* content */*/
#[derive(Accounts)]
pub struct Initialize {}

pub fn processor(_ctx: Context<Initialize>) -> Result<()> {
	Ok(())
}
/*/* content */*/