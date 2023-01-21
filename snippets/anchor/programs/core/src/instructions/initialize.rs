use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn processor(_ctx: Context<Initialize>) -> Result<()> {
	Ok(())
}