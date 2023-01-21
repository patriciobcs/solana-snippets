use anchor_lang::prelude::*;

//* title: Get Bump
//* description: Get the bump of an account
//* platform: anchor
//* category: system
//* prefix: gbump

#[account]
pub struct PDA {
  pub bump: u8,
}

#[derive(Accounts)]
pub struct GetBump<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
      init,
      payer = user,
      space = 8 + 1, 
      seeds = [b"__pda__"], 
      bump
  )]
  pub __pda__: Account<'info, PDA>,
  pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<GetBump>) -> Result<()> {
    let __pda__ = &mut ctx.accounts.__pda__;

    /*/* content */*/
    __pda__.bump = *ctx.bumps.get("__pda__").unwrap();
    /*/* content */*/
    Ok(())
}