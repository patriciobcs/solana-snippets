use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 1, 
        seeds = [b"demo"], 
        bump
    )]
    pub demo: Account<'info, Demo>,
    pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<Initialize>) -> Result<()> {
    let demo = &mut ctx.accounts.demo;
    demo.bump = *ctx.bumps.get("demo").unwrap();
    Ok(())
}