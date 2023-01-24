use anchor_lang::prelude::*;

//* title: Context Init PDA
//* description: Initialize a PDA from an instruction context
//* platform: anchor
//* category: system
//* display: vscode

#[account]
pub struct PDA {
  pub bump: u8,
}

#[derive(Accounts)]
pub struct ContextInitPDA<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /*/* content */*/
    #[account(
        init,
        payer = user,
        space = 8 + __len__, 
        seeds = [b"__pda__"], 
        bump
    )]
    pub __pda__: Account<'info, __PDA__>,
    /*/* content */*/
    pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<ContextInitPDA>) -> Result<()> {
    Ok(())
}